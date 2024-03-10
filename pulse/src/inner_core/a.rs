pub struct Expression<'a> {
    pub data: &'a [u8],      // The heavens
    pub quality: Quality,   // The earth
}

pub type Quality = f64; // Light and darkness