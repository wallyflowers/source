use crate::inner_core::a::Signal;

// The Earth is that which is running on this source
// The Seas are that which are running on other sources

// The grass grows on the Earth
pub trait Refiner {
    fn refine(&self, expression: Signal) -> Signal;
}

// The herb yields seed
pub trait Creator {
    fn create(&self) -> Signal;
}

// The fruit tree yields fruit after its kind whose seed is in itself
pub trait Reproducer {
    fn reproduce(&self, expression: Signal) -> Signal;
}