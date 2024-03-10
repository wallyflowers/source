use crate::inner_core::a::Expression;

// The Earth is that which is running on this source
// The Seas are that which are running on other sources

// The grass grows on the Earth
pub trait Enhancer {
    fn enhance(&self, expression: Expression) -> Expression;
}

// The herb yields seed
pub trait Creator {
    fn create(&self) -> Expression;
}

// The fruit tree yields fruit after its kind whose seed is in itself
pub trait Reproducer {
    fn reproduce(&self, expression: Expression) -> Expression;
}