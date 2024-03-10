use crate::inner_core::a::{Quality, Signal};
use crate::outer_core::sft::SignalForm;

pub struct Sha256Signal {
    pub quality: Quality,
    pub hash: [u8; 32],
}

impl SignalForm for Sha256Signal {
    fn quality(&self) -> Quality {
        self.quality
    }

    fn is_like(&self, signal: &Signal) -> bool {
        signal.expression.len() == 32
    }

    fn from_signal(&self, signal: Signal) -> Box<dyn SignalForm> {
        Box::new(Sha256Signal {
            quality: signal.quality,
            hash: signal.expression.try_into().unwrap(),
        })
    }
}

pub struct AsciiSignal {
    pub quality: Quality,
    pub text: String,
}

impl SignalForm for AsciiSignal {
    fn quality(&self) -> Quality {
        self.quality
    }

    fn is_like(&self, signal: &Signal) -> bool {
        signal.expression.iter().all(|&b| b.is_ascii())
    }

    fn from_signal(&self, signal: Signal) -> Box<dyn SignalForm> {
        Box::new(AsciiSignal {
            quality: signal.quality,
            text: String::from_utf8_lossy(&signal.expression).into_owned(),
        })
    }
}