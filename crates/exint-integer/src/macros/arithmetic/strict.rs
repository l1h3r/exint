macro_rules! strict {
  (core, $name:ident, $uint:expr) => {
    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_add(self, rhs: Self) -> Self {
      panic!("strict_add")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_div(self, rhs: Self) -> Self {
      panic!("strict_div")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_div_euclid(self, rhs: Self) -> Self {
      panic!("strict_div_euclid")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_mul(self, rhs: Self) -> Self {
      panic!("strict_mul")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_neg(self) -> Self {
      panic!("strict_neg")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_pow(self, exp: u32) -> Self {
      panic!("strict_pow")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_rem(self, rhs: Self) -> Self {
      panic!("strict_rem")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_rem_euclid(self, rhs: Self) -> Self {
      panic!("strict_rem_euclid")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_shl(self, rhs: u32) -> Self {
      panic!("strict_shl")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_shr(self, rhs: u32) -> Self {
      panic!("strict_shr")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_sub(self, rhs: Self) -> Self {
      panic!("strict_sub")
    }
  };
  (uint) => {
    $crate::macros::strict!(core, uint, true);

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_add_signed(self, rhs: $crate::int<S>) -> Self {
      panic!("strict_add_signed")
    }
  };
  (int) => {
    $crate::macros::strict!(core, int, false);

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_abs(self) -> Self {
      panic!("strict_abs")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("strict_add_unsigned")
    }

    #[cfg(feature = "strict_overflow_ops")]
    #[doc(cfg(feature = "strict_overflow_ops"))]
    pub const fn strict_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("strict_sub_unsigned")
    }
  };
}

pub(crate) use strict;
