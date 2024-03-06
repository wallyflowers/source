use serde::{Serialize, Deserialize};
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;
use rsa::RsaPrivateKey;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde_json::to_vec;

#[derive(Serialize, Deserialize, Debug)]
struct Signal {
    hash: String, // SHA-256 hash
    quality: f64, // Data Quality
}

#[derive(Serialize, Deserialize, Debug)]
struct Broadcast {
    origin: SocketAddr,     // Originating SocketAddr
    pub_key: Vec<u8>,       // Serialized public RSA key
    signals: Vec<Signal>,   // Collection of Signals
}


struct Node {
    address: SocketAddr,            // Socket address of the node
    rsa_key_pair: RsaPrivateKey,    // RSA private key (public key can be derived)
    broadcast_pool: BroadcastPool,  // A data structure containing all unprocessed broadcasts
}

struct BroadcastPool {
    sender: Sender<Broadcast>, // Used to send broadcasts to the pool
    receiver: Receiver<Broadcast>, // Used to receive and process broadcasts
}

impl Node {
    // Constructs a new Node with a unique RSA key pair
    pub fn new(address: SocketAddr) -> Self {
        let bits = 2048; // Key size for RSA
        let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits)
            .expect("Failed to generate a key");
        let broadcast_pool = BroadcastPool::new();
        Node {
            address,
            rsa_key_pair: private_key,
            broadcast_pool: broadcast_pool,
        }
    }

    pub async fn send_broadcast(&self, broadcast: &Broadcast, destination: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize the broadcast using serde_json
        let serialized = to_vec(broadcast)?;
        
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
                let broadcast: Broadcast = serde_json::from_slice(&buffer[..size]).expect("Failed to deserialize");
                pool.add_broadcast(broadcast);
            });
        }
    }
}

impl BroadcastPool {
    fn new() -> Self {
        let (sender, receiver) = unbounded::<Broadcast>();
        BroadcastPool { sender, receiver }
    }

    fn clone() -> Self {
        //stub
    }

    fn add_broadcast(&self, broadcast: Broadcast) {
        self.sender.send(broadcast).expect("Failed to send broadcast");
    }

    fn start_processing(&self) {
        let receiver = self.receiver.clone();
        thread::spawn(move || {
            for broadcast in receiver.iter() {
                // Process the broadcast
            }
        });
    }
}