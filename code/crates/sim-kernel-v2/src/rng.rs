/// SplitMix64 deterministic primitive. Not cryptographic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[must_use]
pub fn derive_stream_seed(world_seed: u64, domain: &str, entity_key: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64 ^ world_seed;
    for byte in domain.bytes().chain([0xff]).chain(entity_key.bytes()) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::{derive_stream_seed, SplitMix64};

    #[test]
    fn deterministic_stream_is_repeatable() {
        let seed = derive_stream_seed(42, "piracy", "shipment.7");
        let mut a = SplitMix64::new(seed);
        let mut b = SplitMix64::new(seed);
        assert_eq!([a.next_u64(), a.next_u64()], [b.next_u64(), b.next_u64()]);
    }

    #[test]
    fn domain_separation_changes_stream() {
        assert_ne!(
            derive_stream_seed(42, "weather", "shipment.7"),
            derive_stream_seed(42, "piracy", "shipment.7")
        );
    }
}
