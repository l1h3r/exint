use crate::types::uint;

impl<const N: usize> uint<N> {
  #[doc = include_doc!(uint, "unbounded_shl")]
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

  #[doc = include_doc!(uint, "unbounded_shr")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub const fn unbounded_shr(self, rhs: u32) -> Self {
    if rhs < Self::BITS {
      // SAFETY: We just ensured that `rhs` is in-range.
      unsafe { self.unchecked_shr(rhs) }
    } else {
      Self::ZERO
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(@uint, test_unbounded_shl, () => {
    assert_unbounded_shift!(unbounded_shl, T, T::P_1, T::MIN);
    assert_unbounded_shift!(unbounded_shl, T, T::P_8, T::MIN);
    assert_unbounded_shift!(unbounded_shl, T, T::P_17, T::MIN);
    assert_unbounded_shift!(unbounded_shl, T, T::MIN, T::MIN);
    assert_unbounded_shift!(unbounded_shl, T, T::MAX, T::MIN);
  });

  test!(@uint, test_unbounded_shr, () => {
    assert_unbounded_shift!(unbounded_shr, T, T::P_1, T::MIN);
    assert_unbounded_shift!(unbounded_shr, T, T::P_8, T::MIN);
    assert_unbounded_shift!(unbounded_shr, T, T::P_17, T::MIN);
    assert_unbounded_shift!(unbounded_shr, T, T::MIN, T::MIN);
    assert_unbounded_shift!(unbounded_shr, T, T::MAX, T::MIN);
  });
}
