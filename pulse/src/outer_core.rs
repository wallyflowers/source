use crate::inner_core::{Boundary, Signal, SignalSequence};
use crossbeam_channel::{Sender, Receiver};
use rsa::RsaPrivateKey;
use crossbeam_channel::unbounded;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::to_vec;

/// Represents a node in the PULSE network.
pub struct Node {
    pub address: SocketAddr,            // The Socket address of the node.
    pub rsa_key_pair: RsaPrivateKey,    // The RSA private key of the node. The public key can be derived from this.
    pub signal_channel: SignalChannel,        // A data structure containing all unprocessed signals for this node.
}

impl Node {
    // Constructs a new Node with a unique RSA key pair
    pub fn new(address: SocketAddr) -> Self {
        let bits = 2048; // Key size for RSA
        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits)
            .expect("Failed to generate a key");
        let signal_channel = SignalChannel::new();
        Node {
            address,
            rsa_key_pair: private_key,
            signal_channel: signal_channel,
        }
    }

    pub async fn send_signal(&self, signal: &Signal, destination: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize the signal using serde_json
        let serialized = to_vec(signal)?;
        
        // Connect to the destination address
        let mut stream = TcpStream::connect(destination).await?;

        // Send the serialized data
        stream.write_all(&serialized).await?;

        Ok(())
    }

    pub async fn listen_for_signals(&self) {
        let listener = TcpListener::bind(self.address).await.expect("Failed to bind");
        loop {
            let (mut stream, _) = listener.accept().await.expect("Failed to accept");
            let channel = self.signal_channel.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                let size = stream.read(&mut buffer).await.expect("Failed to read");
                let signal: Signal = serde_json::from_slice(&buffer[..size]).expect("Failed to deserialize");
                channel.add_signal(signal);
            });
        }
    }

    pub async fn filter_signals(&self, boundary: &dyn Boundary) {
        
    }
}

/// A data structure that can hold signals. Uses crossbeam_channel to send and receive signals.
pub struct SignalChannel {
    pub sender: Sender<Signal>, // Used to send signals to the pool.
    pub receiver: Receiver<Signal>, // Used to receive and process signals from the pool.
}

impl SignalChannel {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<Signal>();
        SignalChannel { sender, receiver }
    }
}

impl Clone for SignalChannel {
    fn clone(&self) -> Self {
        let (sender, receiver) = unbounded::<Signal>();
        SignalChannel { sender, receiver }
    }
}

impl SignalSequence for SignalChannel {
    fn add_signal(&self, signal: Signal) {
        self.sender.send(signal).expect("Failed to send signal");
    }

    fn read_signal(&self) -> Option<Signal> {
        self.receiver.try_recv().ok()
    }
}