use crate::inner_core::Graph;

pub trait SignalForm {
    fn get_quality(&self) -> Quality;
    fn get_owned_vector(&self) -> Vec<u8>;
}

pub trait SignalBranch {
    fn is_like(&self, signal: &Signal) -> bool;
}

pub trait SignalLeaf {
    fn new(&self, signal: Signal) -> Box<dyn SignalForm>;
}

pub enum SignalFormTree {
    Branch(Box<dyn Fn(&Signal) -> bool>, Vec<SignalFormTree>),
    Leaf(Box<dyn Fn(&Signal) -> Box<dyn SignalForm>>),
}

impl SignalFormTree {
    pub fn process(&self, signal: &Signal) -> Option<Box<dyn SignalForm>> {
        match self {
            SignalFormTree::Branch(condition, branches) => {
                if condition(&signal) {
                    for branch in branches {
                        if let Some(form) = branch.process(signal) {
                            return Some(form);
                        }
                    }
                }
                None
            }
            SignalFormTree::Leaf(action) => Some(action(signal)),
        }
    }
}