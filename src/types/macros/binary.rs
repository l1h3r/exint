macro_rules! binary {
  (int) => {
    $crate::types::macros::binary!(@core, int);

    #[doc = include_doc!(int, "highest_one")]
    #[cfg(feature = "int_lowest_highest_one")]
    #[must_use = must_use_doc!()]
    #[inline(always)]
    pub const fn highest_one(self) -> ::core::option::Option<u32> {
      self.cast_unsigned().highest_one()
    }

    #[doc = include_doc!(int, "lowest_one")]
    #[cfg(feature = "int_lowest_highest_one")]
    #[must_use = must_use_doc!()]
    #[inline(always)]
    pub const fn lowest_one(self) -> ::core::option::Option<u32> {
      self.cast_unsigned().lowest_one()
    }
  };
  (uint) => {
    $crate::types::macros::binary!(@core, uint);

    #[doc = include_doc!(uint, "highest_one")]
    #[cfg(feature = "int_lowest_highest_one")]
    #[must_use = must_use_doc!()]
    #[inline(always)]
    pub const fn highest_one(self) -> ::core::option::Option<u32> {
      if $crate::llapi::unlikely(self.is_zero()) {
        ::core::option::Option::None
      } else {
        ::core::option::Option::Some(Self::BITS - 1 - self.leading_zeros())
      }
    }

    #[doc = include_doc!(uint, "lowest_one")]
    #[cfg(feature = "int_lowest_highest_one")]
    #[must_use = must_use_doc!()]
    #[inline(always)]
    pub const fn lowest_one(self) -> ::core::option::Option<u32> {
      if $crate::llapi::unlikely(self.is_zero()) {
        ::core::option::Option::None
      } else {
        ::core::option::Option::Some(self.trailing_zeros())
      }
    }

    stability! {
      #[unstable(feature = "uint_bit_width")]
      #[doc = include_doc!(uint, "bit_width")]
      #[must_use = must_use_doc!()]
      #[inline(always)]
      pub const fn bit_width(self) -> u32 {
        Self::BITS - self.leading_zeros()
      }
    }
  };
  (@core, $name:ident) => {
    #[doc = include_doc!($name, "reverse_bits")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
      $crate::llapi::swap1::<Self, N>(self)
    }

    #[doc = include_doc!($name, "swap_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
      $crate::llapi::swap8::<Self, N>(self)
    }

    #[doc = include_doc!($name, "rotate_left")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn rotate_left(self, bits: u32) -> Self {
      $crate::llapi::rotl::<Self, N>(self, bits)
    }

    #[doc = include_doc!($name, "rotate_right")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn rotate_right(self, bits: u32) -> Self {
      $crate::llapi::rotr::<Self, N>(self, bits)
    }

    #[doc(alias = "popcount")]
    #[doc(alias = "popcnt")]
    #[doc = include_doc!($name, "count_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn count_ones(self) -> u32 {
      $crate::llapi::ctpop::<Self, N>(self)
    }

    #[doc = include_doc!($name, "count_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn count_zeros(self) -> u32 {
      self.const_not().count_ones()
    }

    #[doc = include_doc!($name, "leading_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn leading_ones(self) -> u32 {
      self.const_not().leading_zeros()
    }

    #[doc = include_doc!($name, "leading_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
      $crate::llapi::ctlz::<Self, N>(self)
    }

    #[doc = include_doc!($name, "trailing_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn trailing_ones(self) -> u32 {
      self.const_not().trailing_zeros()
    }

    #[doc = include_doc!($name, "trailing_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
      $crate::llapi::cttz::<Self, N>(self)
    }

    #[doc = include_doc!($name, "isolate_highest_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn isolate_highest_one(self) -> Self {
      Self::ONE
        .const_shl(Self::BITS - 1)
        .wrapping_shr(self.leading_zeros())
        .const_band(self)
    }

    #[doc = include_doc!($name, "isolate_lowest_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn isolate_lowest_one(self) -> Self {
      self.const_band(self.wrapping_neg())
    }
  };
  ($outer:ident, $inner:ident) => {
    #[doc = include_doc!($outer, $inner, "reverse_bits")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
      Self(self.0.reverse_bits())
    }

    #[doc = include_doc!($outer, $inner, "swap_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
      Self(self.0.swap_bytes())
    }

    #[doc = include_doc!($outer, $inner, "rotate_left")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn rotate_left(self, bits: u32) -> Self {
      Self(self.0.rotate_left(bits))
    }

    #[doc = include_doc!($outer, $inner, "rotate_right")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn rotate_right(self, bits: u32) -> Self {
      Self(self.0.rotate_right(bits))
    }

    #[doc(alias = "popcount")]
    #[doc(alias = "popcnt")]
    #[doc = include_doc!($outer, $inner, "count_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn count_ones(self) -> u32 {
      self.0.count_ones()
    }

    #[doc = include_doc!($outer, $inner, "count_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn count_zeros(self) -> u32 {
      self.0.count_zeros()
    }

    #[doc = include_doc!($outer, $inner, "leading_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn leading_ones(self) -> u32 {
      self.0.leading_ones()
    }

    #[doc = include_doc!($outer, $inner, "leading_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
      self.0.leading_zeros()
    }

    #[doc = include_doc!($outer, $inner, "trailing_ones")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn trailing_ones(self) -> u32 {
      self.0.trailing_ones()
    }

    #[doc = include_doc!($outer, $inner, "trailing_zeros")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
      self.0.trailing_zeros()
    }

    #[doc = include_doc!($outer, $inner, "isolate_highest_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn isolate_highest_one(self) -> Self {
      Self(self.0.isolate_highest_one())
    }

    #[doc = include_doc!($outer, $inner, "isolate_lowest_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn isolate_lowest_one(self) -> Self {
      Self(self.0.isolate_lowest_one())
    }
  };
}

pub(crate) use binary;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(test_reverse_bits, () => {
    assert_eq!(T::B_1.reverse_bits().reverse_bits(), T::B_1);
    assert_eq!(T::B_2.reverse_bits().reverse_bits(), T::B_2);
    assert_eq!(T::B_3.reverse_bits().reverse_bits(), T::B_3);
    assert_eq!(T::P_0.reverse_bits(), T::P_0);
    assert_eq!(T::N_1.reverse_bits(), T::N_1);
  });

  test!(test_swap_bytes, () => {
    assert_eq!(T::B_1.swap_bytes().swap_bytes(), T::B_1);
    assert_eq!(T::B_2.swap_bytes().swap_bytes(), T::B_2);
    assert_eq!(T::B_3.swap_bytes().swap_bytes(), T::B_3);
    assert_eq!(T::P_0.swap_bytes(), T::P_0);
    assert_eq!(T::N_1.swap_bytes(), T::N_1);
  });

  test!(test_rotate_left, () => {
    assert_eq!(T::B_1.rotate_left(3).rotate_left(2).rotate_left(T::BITS - 5), T::B_1);
    assert_eq!(T::B_2.rotate_left(3).rotate_left(2).rotate_left(T::BITS - 5), T::B_2);
    assert_eq!(T::B_3.rotate_left(3).rotate_left(2).rotate_left(T::BITS - 5), T::B_3);

    for index in 0..T::BITS {
      assert_eq!(T::B_1.rotate_left(T::BITS * index), T::B_1);
      assert_eq!(T::B_2.rotate_left(T::BITS * index), T::B_2);
      assert_eq!(T::B_3.rotate_left(T::BITS * index), T::B_3);
      assert_eq!(T::P_0.rotate_left(index), T::P_0);
      assert_eq!(T::N_1.rotate_left(index), T::N_1);
    }
  });

  test!(test_rotate_right, () => {
    assert_eq!(T::B_1.rotate_right(3).rotate_right(2).rotate_right(T::BITS - 5), T::B_1);
    assert_eq!(T::B_2.rotate_right(3).rotate_right(2).rotate_right(T::BITS - 5), T::B_2);
    assert_eq!(T::B_3.rotate_right(3).rotate_right(2).rotate_right(T::BITS - 5), T::B_3);

    for index in 0..T::BITS {
      assert_eq!(T::B_1.rotate_right(T::BITS * index), T::B_1);
      assert_eq!(T::B_2.rotate_right(T::BITS * index), T::B_2);
      assert_eq!(T::B_3.rotate_right(T::BITS * index), T::B_3);
      assert_eq!(T::P_0.rotate_right(index), T::P_0);
      assert_eq!(T::N_1.rotate_right(index), T::N_1);
    }
  });

  test!(test_count_ones, () => {
    assert_eq!(T::B_1.count_ones(), 3);
    assert_eq!(T::B_2.count_ones(), 2);
    assert_eq!(T::B_3.count_ones(), 5);
    assert_eq!(T::P_0.count_ones(), 0);
    assert_eq!(T::N_1.count_ones(), T::BITS);

    assert_eq!((!T::B_1).count_ones(), T::BITS - 3);
    assert_eq!((!T::B_2).count_ones(), T::BITS - 2);
    assert_eq!((!T::B_3).count_ones(), T::BITS - 5);
  });

  test!(test_count_zeros, () => {
    assert_eq!(T::B_1.count_zeros(), T::BITS - 3);
    assert_eq!(T::B_2.count_zeros(), T::BITS - 2);
    assert_eq!(T::B_3.count_zeros(), T::BITS - 5);
    assert_eq!(T::P_0.count_zeros(), T::BITS);
    assert_eq!(T::N_1.count_zeros(), 0);

    assert_eq!((!T::B_1).count_zeros(), 3);
    assert_eq!((!T::B_2).count_zeros(), 2);
    assert_eq!((!T::B_3).count_zeros(), 5);
  });

  test!(test_leading_ones, () => {
    assert_eq!(T::B_1.leading_ones(), 0);
    assert_eq!(T::B_2.leading_ones(), 0);
    assert_eq!(T::B_3.leading_ones(), 0);
    assert_eq!(T::P_0.leading_ones(), 0);
    assert_eq!(T::N_1.leading_ones(), T::BITS);

    assert_eq!((!T::B_1).leading_ones(), T::BITS - 6);
    assert_eq!((!T::B_2).leading_ones(), T::BITS - 6);
    assert_eq!((!T::B_3).leading_ones(), T::BITS - 7);
  });

  test!(test_leading_zeros, () => {
    assert_eq!(T::B_1.leading_zeros(), T::BITS - u8::BITS + 2);
    assert_eq!(T::B_2.leading_zeros(), T::BITS - u8::BITS + 2);
    assert_eq!(T::B_3.leading_zeros(), T::BITS - u8::BITS + 1);
    assert_eq!(T::P_0.leading_zeros(), T::BITS);
    assert_eq!(T::N_1.leading_zeros(), 0);

    assert_eq!((!T::B_1).leading_zeros(), 0);
    assert_eq!((!T::B_2).leading_zeros(), 0);
    assert_eq!((!T::B_3).leading_zeros(), 0);
  });

  test!(test_trailing_ones, () => {
    assert_eq!(T::B_1.trailing_ones(), 0);
    assert_eq!(T::B_2.trailing_ones(), 1);
    assert_eq!(T::B_3.trailing_ones(), 1);
    assert_eq!(T::P_0.trailing_ones(), 0);
    assert_eq!(T::N_1.trailing_ones(), T::BITS);

    assert_eq!((!T::B_1).trailing_ones(), 2);
    assert_eq!((!T::B_2).trailing_ones(), 0);
    assert_eq!((!T::B_3).trailing_ones(), 0);
  });

  test!(test_trailing_zeros, () => {
    assert_eq!(T::B_1.trailing_zeros(), 2);
    assert_eq!(T::B_2.trailing_zeros(), 0);
    assert_eq!(T::B_3.trailing_zeros(), 0);
    assert_eq!(T::P_0.trailing_zeros(), T::BITS);
    assert_eq!(T::N_1.trailing_zeros(), 0);

    assert_eq!((!T::B_1).trailing_zeros(), 0);
    assert_eq!((!T::B_2).trailing_zeros(), 1);
    assert_eq!((!T::B_3).trailing_zeros(), 1);
  });

  test!(test_isolate_highest_one, () => {
    for index in 0..T::BITS {
      let this: T = T::N_1.checked_shr(index).unwrap();
      let that: T = T::MS1.checked_shr(index).unwrap();

      assert_eq!(this.isolate_highest_one(), that.isolate_highest_one());
    }
  });

  test!(test_isolate_lowest_one, () => {
    for index in 0..T::BITS {
      let this: T = T::N_1.checked_shl(index).unwrap();
      let that: T = T::P_1.checked_shl(index).unwrap();

      assert_eq!(this.isolate_lowest_one(), that.isolate_lowest_one());
    }
  });

  test!(@uint, test_highest_one_uint, () => {
    assert_eq!(T::P_0.highest_one(), None);

    for index in 0..T::BITS {
      assert_eq!((T::P_1 << index).highest_one(), Some(index));
      assert_eq!((T::MAX >> index).highest_one(), Some(T::BITS - index - 1));
      assert_eq!((T::MAX << index).highest_one(), Some(T::BITS - 1));
    }
  });

  test!(@uint, test_lowest_one_uint, () => {
    assert_eq!(T::P_0.lowest_one(), None);

    for index in 0..T::BITS {
      assert_eq!((T::P_1 << index).lowest_one(), Some(index));
      assert_eq!((T::MAX >> index).lowest_one(), Some(0));
      assert_eq!((T::MAX << index).lowest_one(), Some(index));
    }
  });

  test!(@uint, test_bit_width, () => {
    assert_eq!(T::B_1.bit_width(), 6);
    assert_eq!(T::B_2.bit_width(), 6);
    assert_eq!(T::B_3.bit_width(), 7);
    assert_eq!(T::P_0.bit_width(), 0);
    assert_eq!(T::N_1.bit_width(), T::BITS);
  });

  test!(@sint, test_highest_one_sint, () => {
    assert_eq!(T::P_0.highest_one(), None);

    for index in 0..T::BITS {
      assert_eq!((T::P_1 << index).highest_one(), Some(index));
      assert_eq!((T::N_1 << index).highest_one(), Some(T::BITS - 1));

      if index != T::BITS - 1 {
        assert_eq!((T::MAX >> index).highest_one(), Some(T::BITS - index - 2));
      }
    }
  });

  test!(@sint, test_lowest_one_sint, () => {
    assert_eq!(T::P_0.lowest_one(), None);

    for index in 0..T::BITS {
      assert_eq!((T::P_1 << index).lowest_one(), Some(index));
      assert_eq!((T::N_1 << index).lowest_one(), Some(index));

      if index != T::BITS - 1 {
        assert_eq!((T::MAX >> index).lowest_one(), Some(0));
      }
    }
  });
}
