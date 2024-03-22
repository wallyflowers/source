use sha2::{Sha256, Digest};
use bincode;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::interface::{Node, Signal, Socket, Hash, Context, Memory, KnowledgeMap};

impl Node {
    /// Create a new `Node` with an empty `KnowledgeMap`.
    pub fn new() -> Node {
        Node {
            knowledge_map: KnowledgeMap::new(),
            in_neighbors: Vec::new(),
            out_neighbors: Vec::new(),
        }
    }
}

impl Memory for KnowledgeMap {
    /// Commit a signal to the `KnowledgeMap` by copying the signal into the vector of signals at the hash.
    /// 
    /// If the hash does not exist in the `KnowledgeMap`, a new vector is created with the signal.
    fn commit(&mut self, signal: Signal) {
        let signals = self.get_mut(&signal.hash);
        match signals {
            Some(signals) => {
                signals.push(signal);
            }
            None => {
                self.insert(signal.hash, vec![signal]);
            }
        }
    }

    /// Recall from the `KnowledgeMap` a copy of the vector of signals at the hash.
    /// 
    /// If the hash does not exist in the `KnowledgeMap`, `None` is returned.
    fn recall(&self, hash: Hash) -> Option<Vec<Signal>> {
        let signals = self.get(&hash);
        match signals {
            Some(signals) => {
                let signals = signals.to_vec();
                Some(signals)
            }
            None => {
                None
            }
        }
    }
}

/// Request a `Signal::Source` from a `Socket`.
fn request_source(socket: &Socket, hash: &Hash) -> Option<Vec<Context>> {
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        None // TODO implement
    } else {
        None
    }
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

/// A helper function to share a `Signal` with a `Socket`.
fn share(socket: &Socket, signal: &Context) {
    let serialized_signal = bincode::serialize(signal).unwrap();
    let socket_address = format_socket_address(socket);
    if let Ok(mut stream) = TcpStream::connect(socket_address) {
        stream.write_all(&serialized_signal).unwrap();
    }
}