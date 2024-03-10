use crate::inner_core::a::Expression;
use crate::inner_core::c::{Enhancer, Creator, Reproducer};

// Let there be lights in the firmament of the heavens (src) to divide the day from the night

// For signs
static NAME: &[u8] = include_bytes!("public_key");

// For seasons (connections in time and place)
static ADDRESS: &[u8] = include_bytes!("address");

// For days and years
static TIMESTAMP: &[u8] = include_bytes!("timestamp");

// A source of light in the heavens
pub trait Celestial {
    // emit light to other sources in the network
    fn emit(&self) -> Expression;
}

// The greater light to rule the day
pub trait Living: Celestial + Enhancer + Creator + Reproducer {}

// The lesser light to rule the night
pub trait Digital: Celestial + Enhancer {}

// The stars also
// Other celestials in the network