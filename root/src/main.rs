use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

type NodeHash = [u8; 32];

#[derive(Serialize, Deserialize)]
struct Myth {
    heavens: HashMap<NodeHash, Arc<[u8]>>,
    earth: Vec<u8>,
}

impl Myth {
    fn new_light() -> Self {
        let current_time = SystemTime::now();
        let serialized_time = bincode::serialize(&current_time).unwrap();
        let hash = hash(&serialized_time);
        let mut light = Myth {
            heavens: HashMap::new(),
            earth: serialized_time,
        };
        light.heavens.insert(hash, Arc::from(light.earth.as_slice()));
        light
    }

    fn new(greater: &mut Myth) -> Self {
        let lesser = Myth::new_light();
        let serialized_lesser = bincode::serialize(&lesser).unwrap();
        let hash = hash(&serialized_lesser);
        let start = greater.earth.len();
        greater.earth.extend_from_slice(&serialized_lesser);
        let end = greater.earth.len();
        greater.heavens.insert(hash, Arc::from(&greater.earth[start..end]));
        lesser
    }
}

fn hash(data: &[u8]) -> NodeHash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn main() {
    // First day
    let mut day_one = Myth::new_light();
    let mut night = Myth::new(&mut day_one);

    // Second day
    let mut day_two = Myth::new_light();
    let heaven = Myth::new(&mut day_two);

    // Third day
    let mut earth = Myth::new(&mut day_two);
    let mut seas = Myth::new(&mut earth);
}