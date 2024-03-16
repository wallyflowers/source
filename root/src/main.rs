use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::process::{Command, exit};
use std::fs::{File, write, read_to_string, create_dir_all};
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use rsa::{RsaPrivateKey, pkcs1::ToRsaPrivateKey};
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize)]
struct Source {
    private_key: RsaPrivateKey,
    present: SystemTime,
    code: String,
    root: String,
}

#[derive(Serialize, Deserialize)]
struct Node<T: Serialize> {
    data: T,
    timestamp: SystemTime,
    edges: HashMap<String, Duration>,
}

impl<T: Serialize> Node<T> {
    fn hash_node(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize)]
struct Snapshot {
    hash: String,
    timestamp: SystemTime,
}

#[derive(Serialize, Deserialize)]
struct Myth<T: Serialize> {
    source: Source,
    nodes: HashMap<String, Node<T>>,
    snapshots: Vec<Snapshot>,
}

impl<T: Serialize> Myth<T> {
    fn new(root: String) -> Self {
        let code_path = Path::new(&root).join("code");
        let bin_path = Path::new(&root).join("bin");
        let myth_path = Path::new(&root).join("myth");
        let key_path = Path::new(&root).join("key");

        let code = read_to_string(&code_path).unwrap_or_else(|_| String::new());
        let bin = std::fs::read(&bin_path).unwrap_or_else(|_| Vec::new());
        let myth: Option<Myth<T>> = File::open(&myth_path).ok().and_then(|file| serde_json::from_reader(file).ok());
        let private_key = File::open(&key_path).ok().and_then(|file| serde_json::from_reader(file).ok())
            .unwrap_or_else(|| {
                let mut rng = OsRng;
                let key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate RSA key");
                let serialized_key = serde_json::to_string(&key).unwrap();
                write(&key_path, serialized_key).expect("Failed to save key to file");
                key
            });

        let present = SystemTime::now();

        myth.unwrap_or_else(|| Myth {
            source: Source { private_key, present, code, root },
            nodes: HashMap::new(),
            snapshots: Vec::new(),
        })
    }

    fn add_node(&mut self, data: T) -> String {
        let timestamp = self.source.present;
        let node = Node {
            data,
            timestamp,
            edges: HashMap::new(),
        };
        let hash = node.hash_node();
        self.nodes.insert(hash.clone(), node);
        hash
    }

    fn restart(&mut self) {
        self.hash_graph();
        self.save_to_disk();
        self.spawn_new_process();
        exit(0);
    }

    fn hash_graph(&mut self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        let hash = format!("{:x}", hasher.finalize());
        let timestamp = self.source.present;
        let snapshot = Snapshot { hash, timestamp };

        if let Some(prev_snapshot) = self.snapshots.last() {
            let weight = timestamp.duration_since(prev_snapshot.timestamp).unwrap();
            self.nodes.get_mut(&prev_snapshot.hash).unwrap().edges.insert(snapshot.hash.clone(), weight);
        }

        self.snapshots.push(snapshot);
    }

    fn save_to_disk(&self) {
        let myth_path = Path::new(&self.source.root).join("myth");
        let code_path = Path::new(&self.source.root).join("code");

        create_dir_all(&self.source.root).expect("Failed to create root directory");

        let serialized_myth = serde_json::to_string(&self).unwrap();
        write(&myth_path, serialized_myth).expect("Failed to save myth to file");

        write(&code_path, &self.source.code).expect("Failed to save code to file");
    }

    fn spawn_new_process(&self) {
        let code_path = Path::new(&self.source.root).join("code");
        let bin_path = Path::new(&self.source.root).join("bin");

        let output = Command::new("rustc")
            .arg(&code_path)
            .arg("-o")
            .arg(&bin_path)
            .output()
            .expect("Failed to compile code");

        if !output.status.success() {
            eprintln!("Compilation error: {}", String::from_utf8_lossy(&output.stderr));
            return;
        }

        Command::new(&bin_path)
            .spawn()
            .expect("Failed to start new process");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <root_path>", args[0]);
        exit(1);
    }

    let root_path = &args[1];
    let myth: Myth<String> = Myth::new(root_path.to_string());

    // TODO: Add your desired functionality here
}
