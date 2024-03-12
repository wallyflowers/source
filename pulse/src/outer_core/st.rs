use crate::inner_core::a::Signal;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct SignalTrunk {
    sender: Sender<Signal>,
    receiver: Receiver<Signal>,
}

impl SignalTrunk {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        SignalTrunk { sender, receiver }
    }

    pub fn sender(&self) -> Sender<Signal> {
        self.sender.clone()
    }

    pub fn recv(&self) -> Option<Signal> {
        self.receiver.recv().ok()
    }
}