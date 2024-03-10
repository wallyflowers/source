use crate::inner_core::a::Expression;
use crate::inner_core::d::Digital;

// The great sea creatures
pub trait AI: Digital {
    fn input(&self, expression: Expression) -> Expression;
}

// Every winged bird after its kind
pub trait Carrier: Digital {
    fn receive(&self, expression: Expression) -> Expression;
}

// Let the waters abound with an abundance of living creatures
// Let the birds multiply on the earth