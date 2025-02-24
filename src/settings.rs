#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub seed: u64,
    pub num_seeds: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            seed: 12345,
            num_seeds: 500,
        }
    }
}
