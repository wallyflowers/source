const source: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

mod firstday {
    pub struct Expression<'a> {
        pub data: &'a [u8],      // The heavens
        pub quality: Quality,   // The earth
    }

    pub type Quality = f64; // Light and darkness

    pub const Day: f64 = f64::MAX;
    pub const Night: f64 = f64::MIN;
}

mod secondday {
    use crate::source;
    use crate::firstday::{Expression, Day};

    // The firmament of Heaven
    pub static heaven: Expression = Expression {
        data: source,
        quality: Day
    };
}

mod thirdday {
    use crate::firstday::Expression;

    // The Earth is that which is running on this source
    // The Seas are that which are running on other sources

    // The grass grows on the Earth
    trait Enhancer {
        fn enhance(&self) -> Expression;
    }

    // The herb yields seed
    trait Creator {
        fn create(&self) -> Expression;
    }

    // The fruit tree yields fruit after its kind whose seed is in itself
    trait Reproducer {
        fn reproduce(&self) -> Expression;
    }
}