use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

type Hash = [u8; 32];
type Time = i32;
type Socket = [u8; 6];
type Knowledge = HashMap<Hash, Vec<u8>>;
type Presence = HashMap<Hash, (Socket, Time)>;

trait Memory {
    fn commit(&mut self, knowledge: &Knowledge);
    fn recall(&self, hash: Hash) -> Option<Knowledge>;
}

impl Memory for Knowledge {
    fn commit(&mut self, knowledge: &Knowledge) {
        let serialized_data = bincode::serialize(knowledge).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&serialized_data);
        let hash = hasher.finalize().into();
        self.insert(hash, serialized_data);
    }

    fn recall(&self, hash: Hash) -> Option<Knowledge> {
        let data = self.get(&hash);
        match data {
            Some(data) => {
                Some(bincode::deserialize(data).unwrap())
            }
            None => None,
        }
    }
}

fn share_knowledge(sockets: &[Socket], knowledge: &Knowledge) {
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
            stream.write_all(&[1]).unwrap(); // Prefix for knowledge message
            stream.write_all(&serialized_knowledge).unwrap();
        }
    }
}

fn share_presence(sockets: &[Socket], presence: &Presence) {
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
            stream.write_all(&[0]).unwrap(); // Prefix for presence message
            stream.write_all(&serialized_presence).unwrap();
        }
    }
}

fn listen(port: u16, knowledge: &mut Knowledge, presence: &mut Presence) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut prefix = [0; 1];
            stream.read_exact(&mut prefix).unwrap();

            match prefix[0] {
                1 => {
                    // Knowledge message
                    let mut serialized_knowledge = Vec::new();
                    stream.read_to_end(&mut serialized_knowledge).unwrap();
                    let received_knowledge: Knowledge = bincode::deserialize(&serialized_knowledge).unwrap();
                    knowledge.commit(&received_knowledge);
                }
                0 => {
                    // Presence message
                    let mut serialized_presence = Vec::new();
                    stream.read_to_end(&mut serialized_presence).unwrap();
                    let received_presence: Presence = bincode::deserialize(&serialized_presence).unwrap();
                    presence.extend(received_presence);
                }
                _ => {
                    // Invalid prefix
                    eprintln!("Invalid message prefix");
                }
            }
        }
    }
}

fn main() {
    let mut knowledge = Knowledge::new();
    let mut presence = Presence::new();

    // Start listening for incoming messages
    listen(34, &mut knowledge, &mut presence);
}