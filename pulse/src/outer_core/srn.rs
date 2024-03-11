use crate::inner_core::a::Signal;
use crate::outer_core::st::SignalTrunk;

pub trait SignalRoot {
    fn listen(&self, trunk: SignalTrunk) -> Option<Signal>;
}

pub enum SignalRootNetwork{

}