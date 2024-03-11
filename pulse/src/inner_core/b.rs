use crate::SOURCE;
use crate::inner_core::a::Signal;
use std::sync::Arc;

// The firmament of Heaven
pub fn get_source_signal() -> Signal {
    Signal {
        expression: Arc::clone(&*SOURCE),
        quality: f64::INFINITY,
    }
}