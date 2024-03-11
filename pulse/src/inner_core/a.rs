use std::sync::Arc;

pub struct Signal {
    pub expression: Arc<[u8]>,   // The heavens
    pub quality: Quality,       // The earth
}

pub type Quality = f64;         // Light and darkness