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

type PresenceMap = HashMap<Hash, Vec<(Socket, Time)>>;  // The sockets and times from which a hash was confirmed to be present in knowledge received.
                                                        // "I have seen this hash at these sockets at these times"
type QualityMap = HashMap<Hash, Value>;                 // The quality of the data, as a value between f64::min and f64::max.
                                                        // f64::min = "I believe this hash to be not worth your resources to review."
                                                        // f64::max = "I believe this hash to be this worth your resources to review."
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

fn share_knowledge(sockets: &[Socket], knowledge: &KnowledgeMap) {
    let serialized_knowledge = bincode::serialize(knowledge).unwrap();
    for socket in sockets {
        let socket_address = format!(
            "{}.{}.{}.{}:{}",
            socket[0],
            socket[1],
            socket[2],
            socket[3],
            (socket[4] as u16) << 8 | socket[5] as u16
        );
        if let Ok(mut stream) = TcpStream::connect(socket_address) {
            stream.write_all(&[0, 0]).unwrap(); // Prefix for knowledge message
            stream.write_all(&serialized_knowledge).unwrap();
        }
    }
}

fn share_presence(sockets: &[Socket], presence: &PresenceMap) {
    let serialized_presence = bincode::serialize(presence).unwrap();
    for socket in sockets {
        let socket_address = format!(
            "{}.{}.{}.{}:{}",
            socket[0],
            socket[1],
            socket[2],
            socket[3],
            (socket[4] as u16) << 8 | socket[5] as u16
        );
        if let Ok(mut stream) = TcpStream::connect(socket_address) {
            stream.write_all(&[1]).unwrap(); // Prefix for presence message
            stream.write_all(&serialized_presence).unwrap();
        }
    }
}

fn share_quality(sockets: &[Socket], quality: &QualityMap) {
    let serialized_quality = bincode::serialize(quality).unwrap();
    for socket in sockets {
        let socket_address = format!(
            "{}.{}.{}.{}:{}",
            socket[0],
            socket[1],
            socket[2],
            socket[3],
            (socket[4] as u16) << 8 | socket[5] as u16
        );
        if let Ok(mut stream) = TcpStream::connect(socket_address) {
            stream.write_all(&[0, 1]).unwrap(); // Prefix for quality message
            stream.write_all(&serialized_quality).unwrap();
        }
    }
}

fn listen(port: u16, knowledge: &mut KnowledgeMap, presence: &mut PresenceMap, quality: &mut QualityMap) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut prefix = [0; 2];
            stream.read_exact(&mut prefix).unwrap();

            match prefix {
                [1, _] => {
                    // Presence message
                    let mut serialized_presence = Vec::new();
                    stream.read_to_end(&mut serialized_presence).unwrap();
                    let received_presence: PresenceMap = bincode::deserialize(&serialized_presence).unwrap();
                    presence.extend(received_presence);
                }
                [0, 1] => {
                    // Quality message
                    let mut serialized_quality = Vec::new();
                    stream.read_to_end(&mut serialized_quality).unwrap();
                    let received_quality: QualityMap = bincode::deserialize(&serialized_quality).unwrap();
                    quality.extend(received_quality);
                }
                [0, 0] => {
                    // Knowledge message
                    let mut serialized_knowledge = Vec::new();
                    stream.read_to_end(&mut serialized_knowledge).unwrap();
                    let received_knowledge: KnowledgeMap = bincode::deserialize(&serialized_knowledge).unwrap();
                    knowledge.commit(&received_knowledge);
                }
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

    // Start listening for incoming messages
    listen(34, &mut knowledge_map, &mut presence_map, &mut quality_map);
}