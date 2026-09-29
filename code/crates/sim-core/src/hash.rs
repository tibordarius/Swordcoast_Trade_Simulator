pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xCBF29CE484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001B3);
    }
    hash
}

pub trait StableStateHash {
    fn stable_state_hash(&self) -> u64;
}
