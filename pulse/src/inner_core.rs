use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use rsa::RsaPrivateKey;
use crossbeam_channel::{Sender, Receiver};


/// Represents a signal with a SHA-256 hash and data quality.
#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    hash: String, // The SHA-256 hash of the signal.
    quality: f64, // The quality of the data, represented as a floating point number.
}

/// Represents a broadcast message.
#[derive(Serialize, Deserialize, Debug)]
struct Broadcast {
    origin: SocketAddr,     // The originating SocketAddr of the broadcast.
    pub_key: Vec<u8>,       // The serialized public RSA key associated with the broadcast.
    signals: Vec<Signal>,   // A collection of Signals associated with the broadcast.
}

/// Represents a network node.
pub struct Node {
    pub address: SocketAddr,            // The Socket address of the node.
    pub rsa_key_pair: RsaPrivateKey,    // The RSA private key of the node. The public key can be derived from this.
    pub broadcast_pool: SignalPool,     // A data structure containing all unprocessed signals for this node.
}

/// Represents a pool of broadcasts.
pub struct SignalPool {
    pub sender: Sender<Signal>, // Used to send broadcasts to the pool.
    pub receiver: Receiver<Signal>, // Used to receive and process broadcasts from the pool.
}

/// A trait that defines if a signal is within a "boundary".
pub trait Boundary {
    fn is_within(&self, signal: &Signal) -> bool;
}

/// A trait to qualify, or evaluate a signal.
pub trait Qualifier {
    fn evaluate(&self, signal: &Signal) -> bool;
}