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

    pub fn send(&self, signal: Signal) {
        self.sender.send(signal).unwrap();
    }

    pub fn recv(&self) -> Option<Signal> {
        self.receiver.recv().ok()
    }
}