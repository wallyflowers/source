use crate::SOURCE;
use crate::inner_core::a::Signal;

// The firmament of Heaven
pub static HEAVEN: Signal = Signal {
    expression: SOURCE.to_owned(),
    quality: f64::INFINITY,
};