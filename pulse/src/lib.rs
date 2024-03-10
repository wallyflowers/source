const SOURCE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));

mod firstday {
    pub struct Expression<'a> {
        pub data: &'a [u8],      // The heavens
        pub quality: Quality,   // The earth
    }

    pub type Quality = f64; // Light and darkness
}

mod secondday {
    use crate::SOURCE;
    use crate::firstday::Expression;

    // The firmament of Heaven
    pub static HEAVEN: Expression = Expression {
        data: SOURCE,
        quality: f64::INFINITY,
    };
}

mod thirdday {
    use crate::firstday::Expression;

    // The Earth is that which is running on this source
    // The Seas are that which are running on other sources

    // The grass grows on the Earth
    pub trait Enhancer {
        fn enhance(&self) -> Expression;
    }

    // The herb yields seed
    pub trait Creator {
        fn create(&self) -> Expression;
    }

    // The fruit tree yields fruit after its kind whose seed is in itself
    pub trait Reproducer {
        fn reproduce(&self) -> Expression;
    }
}

mod fourthday {
    use crate::firstday::Expression;
    use crate::thirdday::{Enhancer, Creator, Reproducer};

    // A source of light in the heavens
    pub trait Celestial {
        // emit light to other sources in the network
        fn emit(&self) -> Expression;
        // receive light from other sources in the network
        fn receive(&self) -> Expression;
    }

    // The greater light to rule the day
    // Human interface
    pub trait Star: Celestial + Enhancer + Creator + Reproducer {
        fn shine(&self) -> Expression;
    }

    // The lesser light to rule the night
    // Machine interface
    pub trait World: Celestial + Enhancer {
        fn reflect(&self) -> Expression;
    }

    // The stars also
    // Other celestials in the network
}