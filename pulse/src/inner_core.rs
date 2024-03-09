use serde::{Serialize, Deserialize};
use std::net::SocketAddr;


/// Represents a signal containing the identity and quality of a piece of data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    hash: String, // The SHA-256 hash of the signal.
    quality: f64, // The quality of the data, represented as a floating point number.
    signature: String, // The signature of the signal.
    pub_key: String, // The public key of the sender.
}


/// A trait that defines methods for sending and receiving signals.
pub trait Interface {
    fn send_signal(&self, signal: &Signal, destination: &SocketAddr) -> Result<(), Box<dyn std::error::Error>>;
    fn listen_for_signals(&self);
}

/// A trait that defines methods for adding and removing signals from a sequential data structure.
pub trait SignalSequence {
    fn add_signal(&self, signal: Signal);
    fn read_signal(&self) -> Option<Signal>;
}

/// A trait that defines methods for storing, updating, and removing signals from a data structure.
pub trait SignalDRUM {
    fn drop(&self, signal: Signal);
    fn read(&self) -> Option<Signal>;
    fn update(&self, signal: Signal);
    fn manifest(&self, signal: Signal);
}

/// A trait that defines if a signal is within a "boundary". Can be used to filter signals.
pub trait Boundary {
    fn is_within(&self, signal: &Signal) -> bool;
}

/// A trait to qualify, or evaluate a signal. Defines a quality metric for a signal.
pub trait Qualifier {
    fn evaluate(&self, signal: &Signal) -> f64;
}