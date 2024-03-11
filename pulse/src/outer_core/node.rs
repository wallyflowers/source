use crate::inner_core::a::Signal;
use crate::outer_core::sft::SignalFormTree;
use crate::outer_core::srn::SignalRootNetwork;

pub trait Node {
    fn get_source_signal(&self) -> Signal;
    fn get_sft(&self) -> SignalFormTree;
    fn get_rnw(&self) -> SignalRootNetwork;
}