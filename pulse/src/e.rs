use crate::inner_core::signal::Signal;
use crate::inner_core::d::Digital;

// The great sea creatures
pub trait AI: Digital {
    fn input(&self, expression: Signal) -> Signal;
}

// Every winged bird after its kind
pub trait Carrier: Digital {
    fn receive(&self, expression: Signal) -> Signal;
}

// Let the waters abound with an abundance of living creatures
// Let the birds multiply on the earth