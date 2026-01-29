// this is vibe coded
use rand_core::{RngCore, SeedableRng};

/// Adapter wrapping `oorandom::Rand64` so it can be used with `rand`, `rand_distr`, etc.
#[derive(Debug,Clone,Copy, PartialEq)]
pub struct OoRng64(oorandom::Rand64);

impl SeedableRng for OoRng64 {
    // Rand64 seeds from a u128; expose that as 16 bytes.
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let s = u128::from_le_bytes(seed);
        OoRng64(oorandom::Rand64::new(s))
    }
}

impl RngCore for OoRng64 {
    fn next_u32(&mut self) -> u32 {
        // Derive 32 bits from rand_u64
        (self.0.rand_u64() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0.rand_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl OoRng64 {
    /// Convenience constructor from a u128 seed
    pub fn from_u128(seed: u128) -> Self {
        OoRng64(oorandom::Rand64::new(seed))
    }
    // Convenience constructor from a u64 seed
    pub fn from_u64(seed: u64) -> Self {
        OoRng64(oorandom::Rand64::new(seed.into()))
    }
}
