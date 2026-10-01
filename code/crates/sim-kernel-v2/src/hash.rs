/// Stable FNV-1a 64-bit hash for deterministic checkpoint utilities.
/// Not intended for cryptographic integrity.
#[must_use]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::fnv1a64;

    #[test]
    fn stable_vector() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"toril"), fnv1a64(b"toril"));
        assert_ne!(fnv1a64(b"toril"), fnv1a64(b"Toril"));
    }
}