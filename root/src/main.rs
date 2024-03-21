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

/// A collection of universal types of information about a hash.
/// They provide a way to share knowledge about a hash within a pulse network.
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
    /// A signal that the data is a pyton source file.
    PY(Data),
    /// A signal that the data is a markdown file.
    MD(Data),
    /// A signal that the data is a text file.
    TXT(Data),
}

/// A type to represent any piece of information.
type Data = Vec<u8>;
/// A 48-bit TCP/IP socket.
type Socket = [u8; 6];
/// A 32-bit Unix timestamp used to record the time a signal was received.
type Time = i32;
/// A 256-bit SHA hash representing a location in a memory.
type Hash = [u8; 32];

/// The most basic form of memory.
type KnowledgeMap =  HashMap<Hash, Vec<Signal>>;

/// An interface to the pulse network.
struct Node {
    /// A map to the `Node`'s knowledge.
    knowledge_map: KnowledgeMap,
    /// The "in-neighbors" of a `Node`.
    /// > "I trust these sockets to provide me with signals that are good for me to know."
    in_neighbors: Vec<Socket>,
    /// The "out-neighbors" of a `Node`.
    /// > "These are sockets which trust me to provide them with signals that are good for them to know."
    out_neighbors: Vec<Socket>,
}

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