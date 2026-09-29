use crate::fnv1a64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    const GAMMA: u64 = 0x9E3779B97F4A7C15;

    pub fn new(seed: u64) -> Self { Self { state: seed } }
    pub fn state(&self) -> u64 { self.state }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(Self::GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

pub fn derive_stream_seed(world_seed: u64, namespace: &str) -> u64 {
    let mut bytes = Vec::with_capacity(8 + namespace.len());
    bytes.extend_from_slice(&world_seed.to_le_bytes());
    bytes.extend_from_slice(namespace.as_bytes());
    fnv1a64(&bytes)
}
