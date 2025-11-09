#![feature(random)]

use ::core::random::RandomSource;
use ::oorandom::Rand64;

pub struct BenchRng(Rand64);

impl BenchRng {
  #[inline]
  pub fn from_seed(seed: [u8; 16]) -> Self {
    Self(Rand64::new(u128::from_le_bytes(seed)))
  }

  #[inline]
  fn bytes(&mut self) -> [u8; 8] {
    self.0.rand_u64().to_le_bytes()
  }
}

impl RandomSource for BenchRng {
  #[inline]
  fn fill_bytes(&mut self, bytes: &mut [u8]) {
    let mut split: (&mut [u8], &mut [u8]);
    let mut value: &mut [u8] = bytes;

    while value.len() >= 8 {
      split = value.split_at_mut(8);
      value = split.1;
      split.0.copy_from_slice(&self.bytes());
    }

    if !value.is_empty() {
      value.copy_from_slice(&self.bytes()[..value.len()]);
    }
  }
}
