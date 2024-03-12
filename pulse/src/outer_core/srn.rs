use crate::inner_core::a::Signal;
use crate::outer_core::st::SignalTrunk;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread::{self, JoinHandle};

pub trait SignalRoot: Send + Sync {
    fn listen(&self, sender: Sender<Signal>, stop_signal: &AtomicBool);
}

pub struct SignalRootNetwork<'a> {
    roots: Vec<(Arc<dyn SignalRoot>, Option<JoinHandle<()>>, Arc<AtomicBool>)>,
    trunk: &'a SignalTrunk,
}

impl<'a> SignalRootNetwork<'a> {
    pub fn new(trunk: &'a SignalTrunk) -> Self {
        SignalRootNetwork {
            roots: Vec::new(),
            trunk,
        }
    }

    pub fn add_root(&mut self, root: impl SignalRoot + 'static) {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let root = Arc::new(root);
        self.roots.push((root, None, stop_signal));
    }

    pub fn remove_root(&mut self, index: usize) {
        if let Some((_, handle, stop_signal)) = self.roots.get_mut(index) {
            stop_signal.store(true, Ordering::Relaxed);
            if let Some(handle) = handle.take() {
                handle.join().unwrap();
            }
        }
        self.roots.remove(index);
    }

    pub fn activate_root(&mut self, index: usize) {
        let sender = self.trunk.sender();
        let (root, handle, stop_signal) = self.roots.get_mut(index).unwrap();
        if handle.is_none() {
            let stop_signal_clone = stop_signal.clone();
            let root_clone = root.clone();
            let new_handle = thread::spawn(move || {
                root_clone.listen(sender, &stop_signal_clone);
            });
            *handle = Some(new_handle);
        }
    }

    pub fn deactivate_root(&mut self, index: usize) {
        if let Some((_, handle, stop_signal)) = self.roots.get_mut(index) {
            stop_signal.store(true, Ordering::Relaxed);
            if let Some(handle) = handle.take() {
                handle.join().unwrap();
            }
        }
    }
}