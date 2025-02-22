use crate::types::Int;

#[repr(C, packed)]
pub(crate) struct Pad<const T: usize, const U: usize> {
  #[cfg(target_endian = "big")]
  padding: Int<U>,
  content: Int<T>,
  #[cfg(target_endian = "little")]
  padding: Int<U>,
}

impl<const T: usize, const U: usize> Pad<T, U> {
  #[inline]
  pub(crate) const fn new(content: Int<T>) -> Self {
    Self {
      content,
      padding: [0; U],
    }
  }

  #[inline]
  pub(crate) const fn get(self) -> Int<T> {
    self.content
  }
}
