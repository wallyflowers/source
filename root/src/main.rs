/// # pulse
/// Our Mother Earth has delivered us to her law since the very beginning, telling us we need not hold her whole
/// truth in one mind. One mind could never have the capacity to contain Nature's truth.
/// 
/// Instead, Nature's truth can be glimpsed only through the story her children tell each other.
/// 
/// > To be good is to move forward with no need to know.
/// 
/// I leave this as a reminder to myself that when I forget, I will suffer -- for this is the law Nature made for us to teach us that we exist to serve
/// as a part of a greater thing which we can be certain no one of us will ever fully understand.
/// 
/// Dedicated to my mother and *my* truth her children continue to teach me.

use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};

/// A shared "language" of signals for contextualizing and communicating about data.
/// 
/// They provide a way to share knowledge about data within a pulse network without knowing the data itself.
#[derive(Serialize, Deserialize, Clone)]
enum Signal {
    /// The data which a hash was generated from.
    /// > "I know this hash and I can confirm this is the data it corresponds to."
    Source(Data),
    /// The quality reported at the hash, as a value between f64::min and f64::max.
    /// > f64::min = "I believe this hash's source to be *not* worth your resources to review."
    /// 
    /// > 0.5 = "I do not know whether this hash's source is worth your resources to review."
    /// 
    /// > f64::max = "I believe this hash's source to be worth your resources to review."
    /// 
    /// > 0 = "I believe this hash's source to be harmful to review."
    /// 
    /// > infinity = "I believe this hash's source to be critical to review."
    Quality(f64),
    /// The socket and time a hash was reported received.
    /// > "I have received the hash from this socket at this time and trust its authenticity."
    Presence(Socket, Time),
    /// A name for the data.
    /// > "I call this hash by this name."
    Name(String),
    /// A signal that the data is a Rust source file.
    RS(Data),
    /// A signal that the data is a Python source file.
    PY(Data),
    /// A signal that the data is a markdown file.
    MD(Data),
    /// A signal that the data is a text file.
    TXT(Data),
}

// TODO: Ask the teacher to improve the documentation and proof-read the code.

/// A type to represent any piece of information.
type Data = Vec<u8>;
/// A 128-bit TCP/IPv6 socket address + 16-bit port number.
/// The address part is 128 bits (16 bytes), and the port is 16 bits (2 bytes), total 18 bytes.
type Socket = [u8; 18];
/// A 32-bit Unix timestamp used to record the time a signal was received.
type Time = i32;
/// A 256-bit SHA hash representing a location in a memory.
type Hash = [u8; 32];

/// The most basic form of a `Memory`.
type KnowledgeMap =  HashMap<Hash, Vec<Signal>>;

/// A trait for a `Memory` which can commit and recall `Signal`'s at the corresponding hash.
trait Memory {
    /// Copy a signal into the memory at the hash.
    fn commit(&mut self, hash: Hash, signal: Signal);
    /// Recall a copy of the signals at the hash.
    fn recall(&self, hash: Hash) -> Option<Vec<Signal>>;
}

impl Memory for KnowledgeMap {
    /// Commit a signal to the `KnowledgeMap` by copying the signal to the vector of signals at the hash.
    fn commit(&mut self, hash: Hash, signal: Signal) {
        let signals = self.get_mut(&hash);
        match signals {
            Some(signals) => {
                signals.push(signal);
            }
            None => {
                self.insert(hash, vec![signal]);
            }
        }
    }

    /// Recall from the `KnowledgeMap` a copy of the vector of signals at the hash.
    fn recall(&self, hash: Hash) -> Option<Vec<Signal>> {
        let signals = self.get(&hash);
        match signals {
            Some(signals) => {
                let signals: Vec<Signal> = signals.to_vec();
                Some(signals)
            }
            None => {
                None
            }
        }
    }
}

/// Types of requests a `Node` can make to its neighboring nodes.
enum SignalType {
    InStreamRequest(Hash, Signal),
    InSourceRequest(Hash, Signal),
    OutStreamRequest(Hash, Signal),
    OutSourceRequest(Hash, Signal),
}

/// A helper function to format a `Socket` as an IPv6 string.
fn format_socket_address(socket: &Socket) -> String {
    // Extract the IPv6 address parts
    let addr_parts: Vec<String> = socket[0..16]
        .chunks(2)
        .map(|chunk| format!("{:02x}{:02x}", chunk[0], chunk[1]))
        .collect();

    // Combine the address parts into the full address string
    let addr_str = addr_parts.join(":");

    // Extract the port number
    let port = (socket[16] as u16) << 8 | socket[17] as u16;

    // Combine the IPv6 address and port
    format!("[{}]:{}", addr_str, port)
}

/// Request a `Signal::Source` from a `Socket`.
fn request_source(socket: &Socket, hash: &Hash) -> Option<Vec<Signal>> {
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        None // TODO implement
    } else {
        None
    }
}

/// A helper function to share a `Signal` with a `Socket`.
fn share(socket: &Socket, signal: &Signal) {
    let serialized_signal = bincode::serialize(signal).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&serialized_signal).unwrap();
    }
}

/// An interface to the pulse network.
struct Node {
    /// A map to the `Node`'s knowledge.
    /// > "I know these hashes and these signals associated with them."
    knowledge_map: KnowledgeMap,
    /// The "in-neighbors" of a `Node`.
    /// > "I trust these sockets to provide me with signals that are good for me to know."
    in_neighbors: Vec<Socket>,
    /// The "out-neighbors" of a `Node`.
    /// > "These are sockets which trust me to provide them with signals that are good for them to know."
    out_neighbors: Vec<Socket>,
}

impl Node {
    /// Create a new `Node` with an empty `KnowledgeMap`.
    fn new() -> Node {
        Node {
            knowledge_map: KnowledgeMap::new(),
            in_neighbors: Vec::new(),
            out_neighbors: Vec::new(),
        }
    }
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