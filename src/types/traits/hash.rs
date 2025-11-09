use ::core::hash::Hash;
use ::core::hash::Hasher;
use ::core::mem::size_of_val;
use ::core::slice;

use crate::types::int;
use crate::types::uint;

#[inline]
const fn slice_to_bytes<T>(input: &[T]) -> &[u8] {
  let len: usize = size_of_val(input);
  let ptr: *const u8 = input.as_ptr().cast();
  // SAFETY: `ptr` is valid and aligned, as this code is only used for numeric
  //         primitives which have no padding. The new slice only spans across
  //         `input` and is never mutated, and its total size is the same as
  //         the original `input` so it can't be over `isize::MAX`.
  unsafe { slice::from_raw_parts(ptr, len) }
}

impl<const N: usize> Hash for int<N> {
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    state.write(&self.to_ne_bytes());
  }

  #[inline]
  fn hash_slice<H>(data: &[Self], state: &mut H)
  where
    H: Hasher,
  {
    state.write(slice_to_bytes(data));
  }
}

impl<const N: usize> Hash for uint<N> {
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    state.write(&self.to_ne_bytes());
  }

  #[inline]
  fn hash_slice<H>(data: &[Self], state: &mut H)
  where
    H: Hasher,
  {
    state.write(slice_to_bytes(data));
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::*;

  struct TestHasher(u64);

  impl Hasher for TestHasher {
    fn finish(&self) -> u64 {
      self.0
    }

    fn write(&mut self, bytes: &[u8]) {
      for byte in bytes {
        self.0 += *byte as u64;
      }
    }
  }

  fn hash<T: Hash>(value: T) -> u64 {
    let mut state: TestHasher = TestHasher(0);
    Hash::hash(&value, &mut state);
    Hasher::finish(&state)
  }

  test!(test_hash, () => {
    assert_eq!(hash(T::P_0), 0);
    assert_eq!(hash(T::P_1), 1);
    assert_eq!(hash(T::P_2), 2);
    assert_eq!(hash(T::P_3), 3);
  });
}
