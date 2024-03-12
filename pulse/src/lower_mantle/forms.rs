use crate::inner_core::{Quality, Signal};
use crate::lower_mantle::sft::{SignalForm, SignalBranch, SignalLeaf};


pub struct BinarySignal {
    pub quality: Quality,
    pub data: Vec<u8>,
}

impl SignalForm for BinarySignal {
    fn get_quality(&self) -> Quality {
        self.quality
    }

    fn get_owned_vector(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl SignalLeaf for BinarySignal {
    fn take_form(&self, signal: Signal) -> Box<dyn SignalForm> {
        Box::new(BinarySignal {
            quality: signal.quality,
            data: signal.expression.to_vec(),
        })
    }
}

pub struct Sha256Signal {
    pub quality: Quality,
    pub hash: [u8; 32],
}

impl SignalForm for Sha256Signal {
    fn get_quality(&self) -> Quality {
        self.quality
    }

    fn get_owned_vector(&self) -> Vec<u8> {
        self.hash.to_vec()
    }
}

impl SignalBranch for Sha256Signal {
    fn is_like(&self, signal: &Signal) -> bool {
        signal.expression.len() == 32
    }
}

impl SignalLeaf for Sha256Signal {
    fn take_form(&self, signal: Signal) -> Box<dyn SignalForm> {
        Box::new(Sha256Signal {
            quality: signal.quality,
            hash: signal.expression.as_ref().try_into().unwrap(),
        })
    }
}

pub struct AsciiSignal {
    pub quality: Quality,
    pub text: String,
}

impl SignalForm for AsciiSignal {
    fn get_quality(&self) -> Quality {
        self.quality
    }

    fn get_owned_vector(&self) -> Vec<u8> {
        self.text.as_bytes().to_vec()
    }
}

impl SignalBranch for AsciiSignal {
    fn is_like(&self, signal: &Signal) -> bool {
        signal.expression.iter().all(|&b| b < 128)
    }
}

impl SignalLeaf for AsciiSignal {
    fn take_form(&self, signal: Signal) -> Box<dyn SignalForm> {
        Box::new(AsciiSignal {
            quality: signal.quality,
            text: String::from_utf8(signal.expression.to_vec()).unwrap(),
        })
    }
}