use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use std::process::{Command, exit};
use std::fs::{File, write, read_to_string, create_dir_all};
use std::path::Path;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use rsa::{RsaPrivateKey, pkcs1::FromRsaPrivateKey};
use rand::rngs::OsRng;

type Key = RsaPrivateKey;
type Main = String;
type Root = String;
type MythHash = String;
type Cargo = String;
type NodeHash = String;

#[derive(Serialize, Deserialize)]
struct Node<T: Serialize> {
    data: T,
    edges: HashMap<NodeHash, Duration>,
}

impl<T: Serialize> Node<T> {
    fn hash_data(data: &T) -> NodeHash {
        let serialized = serde_json::to_string(data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize)]
struct Myth<T: Serialize> {
    nodes: HashMap<NodeHash, Node<T>>,
    current_time: NodeHash,
}

impl<T: Serialize> Myth<T> {
    fn new(root: String) -> Self {
        let code_path = Path::new(&root).join("src").join("main.rs");
        let myth_path = Path::new(&root).join("myth");
        let key_path = Path::new(&root).join("key");
        let cargo_path = Path::new(&root).join("Cargo.toml");
    
        let code = read_to_string(&code_path).unwrap_or_else(|_| String::new());
        let cargo = read_to_string(&cargo_path).unwrap_or_else(|_| String::new());
        let myth: Option<Myth<T>> = File::open(&myth_path).ok().and_then(|file| serde_json::from_reader(file).ok());
    
        let mut nodes = HashMap::new();
        let mut current_time = NodeHash::default();
    
        if let Some(loaded_myth) = myth {
            nodes = loaded_myth.nodes;
            current_time = loaded_myth.current_time;
        } else {
            let key = if key_path.exists() {
                let key_data = read_to_string(&key_path).expect("Failed to read key file");
                serde_json::from_str(&key_data).expect("Failed to deserialize key")
            } else {
                let mut rng = OsRng;
                let key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate RSA key");
                let serialized_key = serde_json::to_string(&key).unwrap();
                write(&key_path, serialized_key).expect("Failed to save key to file");
                key
            };
    
            current_time = Self { nodes, current_time }.add_node(SystemTime::now());
            Self { nodes, current_time }.add_node(key);
            Self { nodes, current_time }.add_node(code);
            Self { nodes, current_time }.add_node(root);
            Self { nodes, current_time }.add_node(cargo);
        }
    
        Myth { nodes, current_time }
    }

    fn add_node(&mut self, data: T) -> NodeHash {
        let timestamp = SystemTime::now();
        let timestamp_hash = Node::hash_data(&timestamp);

        if !self.nodes.contains_key(&timestamp_hash) {
            let timestamp_node = Node { data: timestamp, edges: HashMap::new() };
            if let Some(prev_time_node) = self.nodes.get_mut(&self.current_time) {
                let duration = timestamp.duration_since(prev_time_node.data).unwrap();
                prev_time_node.edges.insert(timestamp_hash.clone(), duration);
            }
            self.nodes.insert(timestamp_hash.clone(), timestamp_node);
            self.current_time = timestamp_hash.clone();
        }

        let data_hash = Node::hash_data(&data);
        if !self.nodes.contains_key(&data_hash) {
            let data_node = Node { data, edges: HashMap::new() };
            self.nodes.insert(data_hash.clone(), data_node);
        }
        if let Some(data_node) = self.nodes.get_mut(&data_hash) {
            data_node.edges.insert(self.current_time.clone(), Duration::ZERO);
        }

        data_hash
    }

    fn restart(&mut self) {
        self.add_node(self.hash_graph());
        self.save_to_disk();
        self.spawn_new_process();
        exit(0);
    }

    fn hash_graph(&mut self) -> MythHash {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }

    fn save_to_disk(&self) {
        let myth_path = Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("myth");
        let code_path = Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("src").join("main.rs");
        let cargo_path = Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("Cargo.toml");

        create_dir_all(Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("src")).expect("Failed to create src directory");

        let serialized_myth = serde_json::to_string(&self).unwrap();
        write(&myth_path, serialized_myth).expect("Failed to save myth to file");

        write(&code_path, &self.nodes.values().find(|node| matches!(node.data, Main(_))).unwrap().data).expect("Failed to save code to file");
        write(&cargo_path, &self.nodes.values().find(|node| matches!(node.data, Cargo(_))).unwrap().data).expect("Failed to save cargo to file");
    }

    fn spawn_new_process(&self) {
        let manifest_path = Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("Cargo.toml");

        let output = Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(&manifest_path)
            .output()
            .expect("Failed to compile code");

        if !output.status.success() {
            eprintln!("Compilation error: {}", String::from_utf8_lossy(&output.stderr));
            return;
        }

        let bin_path = Path::new(&self.nodes.values().find(|node| matches!(node.data, Root(_))).unwrap().data).join("target").join("debug").join("myth");

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
    let mut myth: Myth<String> = Myth::new(root_path.to_string());

    // TODO: Add your desired functionality here
}