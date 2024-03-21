use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};

/// A collection of universal types of information about a hash.
/// They provide a way to share knowledge about a hash within a PULSE network.
/// 
/// Nodes can communicate with other nodes through shared signal variants.
#[derive(Serialize, Deserialize, Clone)]
enum Signal {
    /// The data which a hash was generated from.
    /// > "I know this hash and I can confirm this is the data it corresponds to."
    Source(Data),
    /// The quality of the data, as a value between f64::min and f64::max.
    /// > f64::min = "I believe this data to be not worth your resources to review."
    /// 
    /// > 0.5 = "I do not know whether this data is worth your resources to review."
    /// 
    /// > f64::max = "I believe this data to be worth your resources to review."
    /// 
    /// > 0 = "I believe this data to be harmful to review."
    /// 
    /// > infinity = "I believe this data to be critical to review."
    Quality(f64),
    /// The sockets from and times at which a hash is reported to be have been provided.
    /// > "I have received and checked the hash from this sockets at this time. The data does (not) match the hash."
    Presence(Socket, Time, bool),
    /// A name for the data.
    /// > "I call this hash by this name."
    Name(String),
    /// A signal containing a filename for the hash.
    /// > "I store this hash with this file name."
    FileName(String),
    /// A signal that the data is a pyton source file.
    PY(Data),
    /// A signal that the data is a text file.
    TXT(Data),
    /// A signal that the data is a Rust source file.
    RS(Data),
}

type Data = Vec<u8>;        // A piece of data
type Socket = [u8; 6];      // A 48-bit TCP/IP socket
type Time = i32;            // A 32-bit Unix timestamp
type Hash = [u8; 32];       // A 256-bit SHA hash

type KnowledgeMap =  HashMap<Hash, Vec<Signal>>;      // A map that contains context about hashes

type Stream = Vec<Socket>;

// A trait for a memory which can commit and recall signals it remembers about the corresponding hash
trait Memory {
    fn commit(&mut self, hash: Hash, archetype: Signal, data: &Data);
    fn recall(&self, hash: Hash) -> Option<Vec<Signal>>;
}


impl Memory for KnowledgeMap {
    // TODO verify that the hash is the hash of the data

    fn commit(&mut self, hash: Hash, archetype: Signal, data: &Data) {
        let serialized_data = bincode::serialize(data).unwrap();
        let signals = self.get_mut(&hash);
        match signals {
            Some(signals) => {
                signals.push((archetype, serialized_data)); // TODO figure out how to serialize the data
            }
            None => {
                self.insert(hash, vec![(archetype, serialized_data)]);
            }
        }
    }

    fn recall(&self, hash: Hash) -> Option<Vec<Signal>> {
        let contexts = self.get(&hash);
        match contexts {
            Some(contexts) => {
                let contexts: Vec<Signal> = contexts.to_vec();
                Some(contexts)
            }
            None => {
                None
            }
        }
    }
}


// TODO: make share take a stream and a vector of knowledge, send the knowledge to all the sockets in the stream
fn share<T: serde::Serialize>(socket: &Socket, prefix: &[u8; 2], data: &T) {
    let serialized_data = bincode::serialize(data).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(prefix).unwrap();
        stream.write_all(&serialized_data).unwrap();
    }
}

// TODO: Remove the separate knowledge, presence, and quality sharing functions and make a single share function
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

// TODO: make a single request stream function that takes a teacher socket and a student socket
// Send a request to the teacher containing a tuple of the student's socket as Knowledge of type "stream_request".
fn request_presence_stream(teacher: &Socket, student_port: u16) -> Result<(), std::io::Error> {
    let socket_address = format_socket_address(teacher);
    let mut stream = TcpStream::connect(socket_address)?;
    stream.write_all(&[0, 0])?; // Prefix for requesting presence stream
    stream.write_all(&student_port.to_be_bytes())?;
    Ok(())
}

fn request_quality_stream(teacher: &Socket, student_port: u16) -> Result<(), std::io::Error> {
    let socket_address = format_socket_address(teacher);
    let mut stream = TcpStream::connect(socket_address)?;
    stream.write_all(&[0, 1])?; // Prefix for requesting quality stream
    stream.write_all(&student_port.to_be_bytes())?;
    Ok(())
}

// TODO: send a request to the teacher containing the hash of the knowledge the student wants to learn about as Knowledge of type "knowledge_request".
fn request_knowledge(socket: &Socket, hash: Hash) -> Option<Knowledge> {
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&hash).unwrap();
        let mut serialized_knowledge = Vec::new();
        stream.read_to_end(&mut serialized_knowledge).unwrap();
        let knowledge: Knowledge = bincode::deserialize(&serialized_knowledge).unwrap();
        Some(knowledge)
    } else {
        None
    }
}

fn main() {
    let mut knowledge_map = KnowledgeMap::new();
    let mut source_map = KnowledgeMap::new();
    let mut presence_map = PresenceMap::new();
    let mut quality_map = QualityMap::new();

    let mut student: Vec<Socket> = Vec::new();

    // Start listening for incoming messages
    listen(34, &mut knowledge_map, &mut presence_map, &mut quality_map);
}