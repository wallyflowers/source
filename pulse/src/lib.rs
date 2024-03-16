// The graph is the basic unit of information in the pulse network.
// All information processed by the network can be represented as a graph.
mod inner_core{
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use sha2::{Sha256, Digest};

    #[derive(Serialize, Deserialize)]
    pub struct Node<T> {
        data: T,                        // water
        edges: HashMap<String, f64>,    // light
    }

    #[derive(Serialize, Deserialize)]
    pub struct Graph<T: Serialize> {
        nodes: HashMap<String, Node<T>>,
    }

    impl<T: Serialize> Graph<T> {
        fn new() -> Self {
            Graph { nodes: HashMap::new() }
        }

        fn add_node(&mut self, data: T) -> String {
            let hash = calculate_hash(&data);
            let node = Node {
                data,
                edges: HashMap::new(),
            };
            self.nodes.insert(hash.clone(), node);
            hash
        }

        fn add_edge(&mut self, from: &str, to: &str, weight: f64) {
            if let Some(node) = self.nodes.get_mut(from) {
                node.edges.insert(to.to_string(), weight);
            }
        }

        fn get_node(&self, hash: &str) -> Option<&Node<T>> {
            self.nodes.get(hash)
        }

        fn get_nodes(&self) -> Vec<&Node<T>> {
            self.nodes.values().collect()
        }
    }

    fn calculate_hash<T: Serialize>(data: &T) -> String {
        let serialized = serde_json::to_string(data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }
}

mod outer_core{
    use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};
    use rand::rngs::OsRng;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct RsaKeyPairNode {
        private_key: Vec<u8>,
        public_key: Vec<u8>,
    }

    impl RsaKeyPairNode {
        fn new(bit_size: usize) -> Self {
            let mut rng = OsRng;
            let private_key = RsaPrivateKey::new(&mut rng, bit_size).expect("Failed to generate RSA private key");
            let public_key = RsaPublicKey::from(&private_key);

            let private_key_der = private_key.to_pkcs1_der().expect("Failed to encode private key to DER");
            let public_key_der = public_key.to_pkcs1_der().expect("Failed to encode public key to DER");

            RsaKeyPairNode {
                private_key: private_key_der,
                public_key: public_key_der,
            }
        }

        fn get_private_key(&self) -> RsaPrivateKey {
            RsaPrivateKey::from_pkcs1_der(&self.private_key).expect("Failed to decode private key from DER")
        }

        fn get_public_key(&self) -> RsaPublicKey {
            RsaPublicKey::from_pkcs1_der(&self.public_key).expect("Failed to decode public key from DER")
        }

        fn encrypt(&self, data: &[u8]) -> Vec<u8> {
            let public_key = self.get_public_key();
            let mut rng = OsRng;
            public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data).expect("Failed to encrypt data")
        }

        fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
            let private_key = self.get_private_key();
            private_key.decrypt(Pkcs1v15Encrypt, ciphertext).expect("Failed to decrypt ciphertext")
        }
    }
}

mod lower_mantle{
    pub mod node;
    pub mod forms;
    pub mod roots;
    pub mod sft;
    pub mod st;
    pub mod srn;
}

mod upper_mantle{
    pub mod rsa;
    pub mod net;
    pub mod time;
    pub mod source;
    pub mod root;
}