use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

type Hash = [u8; 32];       // A 256-bit SHA hash
type Time = i32;            // A 32-bit Unix timestamp
type Socket = [u8; 6];      // A 48-bit TCP/IP socket
type Value = f64;           // A 64-bit floating point number
type Data = Vec<u8>;        // A variable-length byte array
type Typecode = [u8; 2];    // A 16-bit type code

type Presence = (Hash, Socket, Time);   // The sockets from and times at which a hash was confirmed to be have been provided.
                                        // "I have seen this hash at these sockets at these times"
type Quality = (Hash, Value);           // The quality of the data, as a value between f64::min and f64::max.
                                        // f64::min = "I believe this data to be not worth your resources to review."
                                        // 0.5 = "I am indifferent about this data."
                                        // f64::max = "I believe this data to be worth your resources to review."
                                        // 0 = "I believe this data to be harmful to review."
                                        // infinity = "I believe this data to be critical to review."
type Knowledge = (Hash, Data);          // The hash of the data and the data itself 
                                        // "I know this hash and this is the data"

type PresenceMap = HashMap<Hash, Vec<(Socket, Time)>>;
type QualityMap = HashMap<Hash, Vec<Value>>;
type KnowledgeMap = HashMap<Hash, Data>;

// A trait for a memory which can commit and recall data by hash
trait Memory {
    fn commit(&mut self, data: &Data);
    fn recall(&self, hash: Hash) -> Option<Data>;
}


impl Memory for KnowledgeMap {
    fn commit(&mut self, data: &Data) {
        let serialized_data = bincode::serialize(data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&serialized_data);
        let hash = hasher.finalize().into();
        self.insert(hash, serialized_data);
    }

    fn recall(&self, hash: Hash) -> Option<Data> {
        let data = self.get(&hash);
        match data {
            Some(data) => {
                Some(bincode::deserialize(data).unwrap())
            }
            None => None,
        }
    }
}

fn share<T: serde::Serialize>(socket: &Socket, prefix: &[u8; 2], data: &T) {
    let serialized_data = bincode::serialize(data).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(prefix).unwrap();
        stream.write_all(&serialized_data).unwrap();
    }
}

fn share_knowledge(socket: &Socket, knowledge: &Knowledge) {
    share(socket, &[0, 0], knowledge);
}

fn share_presence(socket: &Socket, presence: &Presence) {
    share(socket, &[1, 0], presence);
}

fn share_quality(socket: &Socket, quality: &Quality) {
    share(socket, &[0, 1], quality);
}

fn format_socket_address(socket: &Socket) -> String {
    format!(
        "{}.{}.{}.{}:{}",
        socket[0], socket[1], socket[2], socket[3],
        (socket[4] as u16) << 8 | socket[5] as u16
    )
}

// Listen for incoming messages and update the knowledge map, presence map, and quality map accordingly
// TODO: Filter out messages
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

// Find a source which your current quality map suggests is the best source for you to listen to
fn find_source(quality_map: QualityMap) -> Option<Socket> {
    // TODO
}

// Find a teacher which your current presence map suggests is the best teacher for a given hash
fn find_teacher(presence_map: PresenceMap, hash: Hash) -> Option<Socket> {
    // TODO
}

fn main() {
    let mut knowledge_map = KnowledgeMap::new();
    let mut source_map = KnowledgeMap::new();
    let mut presence_map = PresenceMap::new();
    let mut quality_map = QualityMap::new();

    let mut pupils: Vec<Socket> = Vec::new();

    // Start listening for incoming messages
    listen(34, &mut knowledge_map, &mut presence_map, &mut quality_map);
}