use ::core::marker::Sized;
use ::core::ops::RangeFull;
use ::core::random::Distribution;
use ::core::random::RandomSource;

use crate::types::Saturating;
use crate::types::Wrapping;
use crate::types::int;
use crate::types::uint;

#[cfg(feature = "random")]
impl<const N: usize> Distribution<int<N>> for RangeFull {
  fn sample(&self, source: &mut (impl RandomSource + ?Sized)) -> int<N> {
    let mut bytes: [u8; N] = [0; N];
    source.fill_bytes(&mut bytes);
    int::from_ne_bytes(bytes)
  }
}

#[cfg(feature = "random")]
impl<const N: usize> Distribution<uint<N>> for RangeFull {
  fn sample(&self, source: &mut (impl RandomSource + ?Sized)) -> uint<N> {
    let mut bytes: [u8; N] = [0; N];
    source.fill_bytes(&mut bytes);
    uint::from_ne_bytes(bytes)
  }
}

#[cfg(feature = "random")]
impl<T> Distribution<Saturating<T>> for RangeFull
where
  RangeFull: Distribution<T>,
{
  #[inline]
  fn sample(&self, source: &mut (impl RandomSource + ?Sized)) -> Saturating<T> {
    Saturating(Distribution::sample(self, source))
  }
}

#[cfg(feature = "random")]
impl<T> Distribution<Wrapping<T>> for RangeFull
where
  RangeFull: Distribution<T>,
{
  #[inline]
  fn sample(&self, source: &mut (impl RandomSource + ?Sized)) -> Wrapping<T> {
    Wrapping(Distribution::sample(self, source))
  }
}
