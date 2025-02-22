//! Common types, for internal use only.

/// Type alias for an integer represented as an array of bytes.
pub(crate) type Integer<const S: usize> = [u8; S];

/// Helper type for optimized implementations of sign/zero-extend operations.
#[repr(C, packed)]
pub(crate) struct Padding<const T: usize, const U: usize> {
  #[cfg(target_endian = "big")]
  padding: Integer<U>,
  integer: Integer<T>,
  #[cfg(target_endian = "little")]
  padding: Integer<U>,
}

impl<const T: usize, const U: usize> Padding<T, U> {
  #[inline]
  pub(crate) const fn new(integer: Integer<T>) -> Self {
    Self {
      integer,
      padding: [0; U],
    }
  }

  #[inline]
  pub(crate) const fn get(self) -> Integer<T> {
    self.integer
  }
}
