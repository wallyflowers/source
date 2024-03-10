const SOURCE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

mod inner_core{
    pub mod a;
    mod b;
    mod c;
    mod d;
    mod e;
    mod f;
}

mod outer_core{
    pub mod sft;
}

mod mantle {
    mod forms;
}