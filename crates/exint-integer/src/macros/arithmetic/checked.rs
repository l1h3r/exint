macro_rules! checked {
  (core, $name:ident, $uint:expr) => {
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
      panic!("checked_add")
    }

    pub const fn checked_div(self, rhs: Self) -> Option<Self> {
      panic!("checked_div")
    }

    pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
      panic!("checked_div_euclid")
    }

    pub const fn checked_ilog(self, base: Self) -> Option<u32> {
      panic!("checked_ilog")
    }

    pub const fn checked_ilog10(self) -> Option<u32> {
      panic!("checked_ilog10")
    }

    pub const fn checked_ilog2(self) -> Option<u32> {
      panic!("checked_ilog2")
    }

    pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
      panic!("checked_mul")
    }

    pub const fn checked_neg(self) -> Option<Self> {
      panic!("checked_neg")
    }

    pub const fn checked_pow(self, exp: u32) -> Option<Self> {
      panic!("checked_pow")
    }

    pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
      panic!("checked_rem")
    }

    pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
      panic!("checked_rem_euclid")
    }

    pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
      panic!("checked_shl")
    }

    pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
      panic!("checked_shr")
    }

    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
      panic!("checked_sub")
    }
  };
  (uint) => {
    $crate::macros::checked!(core, uint, true);

    pub const fn checked_add_signed(self, rhs: $crate::int<S>) -> Option<Self> {
      panic!("checked_add_signed")
    }

    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
      panic!("checked_next_multiple_of")
    }

    pub const fn checked_next_power_of_two(self) -> Option<Self> {
      panic!("checked_next_power_of_two")
    }

    #[cfg(feature = "unsigned_signed_diff")]
    #[doc(cfg(feature = "unsigned_signed_diff"))]
    pub const fn checked_signed_diff(self, rhs: Self) -> Option<$crate::int<S>> {
      panic!("checked_signed_diff")
    }
  };
  (int) => {
    $crate::macros::checked!(core, int, false);

    pub const fn checked_abs(self) -> Option<Self> {
      panic!("checked_abs")
    }

    pub const fn checked_add_unsigned(self, rhs: $crate::uint<S>) -> Option<Self> {
      panic!("checked_add_unsigned")
    }

    #[cfg(feature = "isqrt")]
    #[doc(cfg(feature = "isqrt"))]
    pub const fn checked_isqrt(self) -> Option<Self> {
      panic!("checked_isqrt")
    }

    #[cfg(feature = "int_roundings")]
    #[doc(cfg(feature = "int_roundings"))]
    pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
      panic!("checked_next_multiple_of")
    }

    pub const fn checked_sub_unsigned(self, rhs: $crate::uint<S>) -> Option<Self> {
      panic!("checked_sub_unsigned")
    }
  };
}

pub(crate) use checked;
