mod inner_core{
    use std::sync::Arc;

    pub struct Signal {
        pub expression: Arc<[u8]>,
        pub quality: Quality,
    }

    pub type Quality = f64;
}

mod outer_core{
    use crate::inner_core::Signal;
    use lazy_static::lazy_static;
    use std::sync::Arc;

    lazy_static! {
        static ref SOURCE: Arc<[u8]> = Arc::from(*include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")));
    }

    // The firmament of Heaven
    pub fn get_source_signal() -> Signal {
        Signal {
            expression: Arc::clone(&*SOURCE),
            quality: f64::INFINITY,
        }
    }
}

mod lower_mantle{
    pub mod node;
    pub mod forms;
    pub mod roots;
    pub mod sft;
    pub mod st;
    pub mod srn;
}