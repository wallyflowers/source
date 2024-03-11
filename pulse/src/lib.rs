use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref SOURCE: Arc<[u8]> = Arc::from(*include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")));
}

mod inner_core{
    pub mod a;
    mod b;
    mod c;
    mod d;
    mod e;
    mod f;
}

mod outer_core{
    pub mod node;
    pub mod forms;
    pub mod sft;
    pub mod st;
    pub mod srn;
}