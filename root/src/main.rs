use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

type Hash = [u8; 32];
type Time = i32;
type Socket = [u8; 6];
type Value = f64;
type Data = Vec<u8>;

type Presence = (Hash, Socket, Time);
type Quality = (Hash, Value);
type Knowledge = (Hash, Data);

type PresenceMap = HashMap<Hash, Vec<(Socket, Time)>>;  // The sockets and times from which a hash was confirmed to be present in knowledge received.
                                                        // "I have seen this hash at these sockets at these times"
type QualityMap = HashMap<Hash, Vec<Value>>;            // The quality of the data, as a value between f64::min and f64::max.
                                                        // f64::min = "I believe this hash to be not worth your resources to review."
                                                        // 0.5 = "I am indifferent about this hash."
                                                        // f64::max = "I believe this hash to be worth your resources to review."
                                                        // 0 = "I believe this hash to be harmful to review."
                                                        // infinity = "I believe this hash to be critical to review."
type KnowledgeMap = HashMap<Hash, Data>;                // The hash of the data and the data itself 
                                                        // "I know this hash and this is the data"

trait Memory {
    fn commit(&mut self, knowledge: &KnowledgeMap);
    fn recall(&self, hash: Hash) -> Option<KnowledgeMap>;
}

impl Memory for KnowledgeMap {
    fn commit(&mut self, knowledge: &KnowledgeMap) {
        let serialized_data = bincode::serialize(knowledge).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&serialized_data);
        let hash = hasher.finalize().into();
        self.insert(hash, serialized_data);
    }

    fn recall(&self, hash: Hash) -> Option<KnowledgeMap> {
        let data = self.get(&hash);
        match data {
            Some(data) => {
                Some(bincode::deserialize(data).unwrap())
            }
            None => None,
        }
    }
}

fn share_knowledge(socket: &Socket, knowledge: &Knowledge) {
    let serialized_knowledge = bincode::serialize(knowledge).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&[0, 0]).unwrap(); // Prefix for knowledge message
        stream.write_all(&serialized_knowledge).unwrap();
    }
}

fn share_presence(socket: &Socket, presence: &Presence) {
    let serialized_presence = bincode::serialize(presence).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&[1, 0]).unwrap(); // Adjusted prefix for presence message
        stream.write_all(&serialized_presence).unwrap();
    }
}

fn share_quality(socket: &Socket, quality: &Quality) {
    let serialized_quality = bincode::serialize(quality).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&[0, 1]).unwrap(); // Prefix for quality message
        stream.write_all(&serialized_quality).unwrap();
    }
}

fn format_socket_address(socket: &Socket) -> String {
    format!(
        "{}.{}.{}.{}:{}",
        socket[0], socket[1], socket[2], socket[3],
        (socket[4] as u16) << 8 | socket[5] as u16
    )
}

fn listen(port: u16, knowledge_map: &mut KnowledgeMap, presence_map: &mut PresenceMap, quality_map: &mut QualityMap) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut prefix = [0; 2];
            stream.read_exact(&mut prefix).unwrap();

            match prefix {
                [1, 0] => {
                    // Presence message
                    let mut serialized_presence = Vec::new();
                    stream.read_to_end(&mut serialized_presence).unwrap();
                    let presence: Presence = bincode::deserialize(&serialized_presence).unwrap();
                    presence_map.entry(presence.0).or_insert(Vec::new()).push((presence.1, presence.2));
                },
                [0, 1] => {
                    // Quality message
                    let mut serialized_quality = Vec::new();
                    stream.read_to_end(&mut serialized_quality).unwrap();
                    let quality: Quality = bincode::deserialize(&serialized_quality).unwrap();
                    quality_map.entry(quality.0).or_insert(Vec::new()).push(quality.1);
                },
                [0, 0] => {
                    // Knowledge message
                    let mut serialized_knowledge = Vec::new();
                    stream.read_to_end(&mut serialized_knowledge).unwrap();
                    let knowledge: Knowledge = bincode::deserialize(&serialized_knowledge).unwrap();
                    knowledge_map.insert(knowledge.0, knowledge.1);
                },
                _ => {
                    // Invalid prefix
                    eprintln!("Invalid message prefix");
                }
            }
        }
    }
}

fn find_teacher(presence: &PresenceMap, hash: Hash) -> Option<Socket> {
    // Extract the list of presences for the given hash
    if let Some(presences) = presence.get(&hash) {
        let mut socket_counts = HashMap::new(); // To count each socket's frequency
        let mut socket_times = HashMap::new(); // To track the most recent time for each socket

        for (socket, time) in presences {
            let count = socket_counts.entry(*socket).or_insert(0);
            *count += 1;

            let socket_time = socket_times.entry(*socket).or_insert(0);
            if *time > *socket_time {
                *socket_time = *time; // Update to the most recent time
            }
        }

        // Now, determine the socket with the highest count and most recent time
        socket_counts.into_iter().max_by(|a, b| {
            let &(ref socket_a, count_a) = a;
            let &(ref socket_b, count_b) = b;

            let time_a = *socket_times.get(socket_a).unwrap();
            let time_b = *socket_times.get(socket_b).unwrap();

            count_a.cmp(&count_b) // First, compare by count
                .then_with(|| time_b.cmp(&time_a)) // In case of tie, use the most recent time (reverse order)
        }).map(|(socket, _)| socket)
    } else {
        None
    }
}

fn main() {
    let mut knowledge_map = KnowledgeMap::new();
    let mut presence_map = PresenceMap::new();
    let mut quality_map = QualityMap::new();
    let mut streams = KnowledgeMap::new();

    // Start listening for incoming messages
    listen(34, &mut knowledge_map, &mut presence_map, &mut quality_map);
}