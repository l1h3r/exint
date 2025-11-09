use crate::types::int;

impl<const N: usize> int<N> {
  #[doc = include_doc!(int, "unbounded_shl")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shl(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shl(rhs) }
    } else {
      Self::ZERO
    }
  }

  #[doc = include_doc!(int, "unbounded_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shr(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shr(rhs) }
    } else {
      // SAFETY: `Self::BITS - 1` is guaranteed to be less than `Self::BITS`.
      unsafe { self.unchecked_shr(Self::BITS - 1) }
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@sint, test_unbounded_shl, () => {
    assert_unbounded_shift!(unbounded_shl, T, T::N_1, T::P_0);
    assert_unbounded_shift!(unbounded_shl, T, T::P_1, T::P_0);
    assert_unbounded_shift!(unbounded_shl, T, T::P_8, T::P_0);
    assert_unbounded_shift!(unbounded_shl, T, T::P_17, T::P_0);
    assert_unbounded_shift!(unbounded_shl, T, T::MIN, T::P_0);
    assert_unbounded_shift!(unbounded_shl, T, T::MAX, T::P_0);
  });

  test!(@sint, test_unbounded_shr, () => {
    assert_unbounded_shift!(unbounded_shr, T, T::N_1, T::N_1);
    assert_unbounded_shift!(unbounded_shr, T, T::P_1, T::P_0);
    assert_unbounded_shift!(unbounded_shr, T, T::P_8, T::P_0);
    assert_unbounded_shift!(unbounded_shr, T, T::P_17, T::P_0);
    assert_unbounded_shift!(unbounded_shr, T, T::MIN, T::N_1);
    assert_unbounded_shift!(unbounded_shr, T, T::MAX, T::P_0);
  });
}
