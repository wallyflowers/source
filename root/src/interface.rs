use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A 128-bit TCP/IPv6 socket address + 16-bit port number.
/// The address part is 128 bits (16 bytes), and the port is 16 bits (2 bytes), total 18 bytes.
pub type Socket = [u8; 18];

/// A 256-bit SHA hash representing a location in a memory.
pub type Hash = [u8; 32];

/// The basic unit of information within a pulse network.
/// 
/// A shared language for sharing knowledge about data.
#[derive(Serialize, Deserialize, Clone)]
pub struct Signal {
    pub hash: Hash,
    pub context: Context,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Context {
    /// The data which a hash was generated from.
    /// > "I know this hash and I can confirm this is the data it corresponds to."
    Source(Data),
    /// Metadata about the data which a hash was generated from.
    Meta(Metadata)
}

/// Data which corresponds to the signal hash.
#[derive(Serialize, Deserialize, Clone)]
enum Data {
    /// A signal containing a binary file.
    BLOB(Vec<u8>),
    /// A signal containing a Rust source file.
    RS(Vec<u8>),
    /// A signal containing a Python source file.
    PY(Vec<u8>),
    /// A signal containing a markdown file.
    MD(Vec<u8>),
    /// A signal containing a text file.
    TXT(Vec<u8>),
}

/// Metadata which corresponds to the signal hash.
#[derive(Serialize, Deserialize, Clone)]
enum Metadata {
    /// A name for the data.
    /// > "I call this hash by this name."
    Name(String),
    /// An evaluation of the hash, as a value between f64::min and f64::max.
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
    /// A 128-bit TCP/IPv6 socket address + 16-bit port number.
    /// The address part is 128 bits (16 bytes), and the port is 16 bits (2 bytes), total 18 bytes.
    Socket(Socket),
    /// A 64-bit Unix timestamp used to record the time a signal was received.
    Time(i64),
}

/// A trait for a `Memory` which can commit and recall `Signal`'s at the corresponding hash.
pub trait Memory {
    /// Copy a signal into the memory at the hash.
    fn commit(&mut self, signal: Signal);
    /// Recall a copy of the signals at the hash.
    fn recall(&self, hash: Hash) -> Option<Vec<Signal>>;
}

/// The most basic form of a `Memory`.
/// Everything a `Node` knows about a thing; knowledge about an entity that can be expressed through the network.
pub type KnowledgeMap =  HashMap<Hash, Vec<Signal>>;

/// A map of a `Node`'s neigbors.
pub struct NetworkMap {
    /// The "known nodes" of a `Node`. Each `KnowledgeMap` is a `Node`'s knowledge of one of their neighbors.
    pub known_nodes: Vec<KnowledgeMap>,
}

/// An interface to the pulse network.
pub struct Node {
    /// A map to the `Node`'s internal knowledge.
    /// > "I know these hashes and these signals associated with them."
    pub knowledge_map: KnowledgeMap,
    /// A knowledge map for each of the `Node`'s neighbors.
    pub network_map: NetworkMap,
}