use crate::inner_core::{Signal, Node, SignalPool};
use rsa::RsaPrivateKey;
use crossbeam_channel::unbounded;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::to_vec;
use std::thread;


impl Node {
    // Constructs a new Node with a unique RSA key pair
    pub fn new(address: SocketAddr) -> Self {
        let bits = 2048; // Key size for RSA
        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits)
            .expect("Failed to generate a key");
        let broadcast_pool = SignalPool::new();
        Node {
            address,
            rsa_key_pair: private_key,
            broadcast_pool: broadcast_pool,
        }
    }

    pub async fn send_broadcast(&self, signal: &Signal, destination: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize the broadcast using serde_json
        let serialized = to_vec(signal)?;
        
        // Connect to the destination address
        let mut stream = TcpStream::connect(destination).await?;

        // Send the serialized data
        stream.write_all(&serialized).await?;

        Ok(())
    }

    pub async fn listen_for_broadcasts(&self) {
        let listener = TcpListener::bind(self.address).await.expect("Failed to bind");
        loop {
            let (mut stream, _) = listener.accept().await.expect("Failed to accept");
            let pool = self.broadcast_pool.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                let size = stream.read(&mut buffer).await.expect("Failed to read");
                let signal: Signal = serde_json::from_slice(&buffer[..size]).expect("Failed to deserialize");
                pool.add_signal(signal);
            });
        }
    }
}

impl SignalPool {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<Signal>();
        SignalPool { sender, receiver }
    }

    pub fn clone(&self) -> Self {
        let (sender, receiver) = unbounded::<Signal>();
        SignalPool { sender, receiver }
    }

    pub fn add_signal(&self, signal: Signal) {
        self.sender.send(signal).expect("Failed to send broadcast");
    }

    fn start_processing(&self) {
        let receiver = self.receiver.clone();
        thread::spawn(move || {
            for broadcast in receiver.iter() {
                println!("{:?}", broadcast);
            }
        });
    }
}