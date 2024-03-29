use crate::inner_core::Signal;
use crate::lower_mantle::sft::SignalFormTree;
use crate::lower_mantle::srn::SignalRootNetwork;

pub trait Node {
    fn get_source_signal(&self) -> Signal;
    fn get_sft(&self) -> SignalFormTree;
    fn get_rnw(&self) -> SignalRootNetwork;
}