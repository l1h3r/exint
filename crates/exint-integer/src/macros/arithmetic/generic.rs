macro_rules! generic {
  (core, $name:ident, $uint:expr) => {
    pub const fn abs_diff(self, other: Self) -> Self {
      panic!("abs_diff")
    }

    pub const fn div_euclid(self, rhs: Self) -> Self {
      panic!("div_euclid")
    }

    #[cfg(feature = "int_roundings")]
    #[doc(cfg(feature = "int_roundings"))]
    pub const fn div_floor(self, rhs: Self) -> Self {
      panic!("div_floor")
    }

    pub const fn ilog(self, base: Self) -> u32 {
      panic!("ilog")
    }

    pub const fn ilog10(self) -> u32 {
      panic!("ilog10")
    }

    pub const fn ilog2(self) -> u32 {
      panic!("ilog2")
    }

    #[cfg(feature = "isqrt")]
    #[doc(cfg(feature = "isqrt"))]
    pub const fn isqrt(self) -> Self {
      panic!("isqrt")
    }

    #[deprecated(note = "replaced by the `MAX` associated constant on this type")]
    pub const fn max_value() -> Self {
      panic!("max_value")
    }

    #[cfg(feature = "num_midpoint")]
    #[doc(cfg(feature = "num_midpoint"))]
    pub const fn midpoint(self, rhs: Self) -> Self {
      panic!("midpoint")
    }

    #[deprecated(note = "replaced by the `MIN` associated constant on this type")]
    pub const fn min_value() -> Self {
      panic!("min_value")
    }

    pub const fn pow(self, exp: u32) -> Self {
      panic!("pow")
    }

    pub const fn rem_euclid(self, rhs: Self) -> Self {
      panic!("rem_euclid")
    }
  };
  (uint) => {
    $crate::macros::generic!(core, uint, true);

    #[cfg(feature = "integer_sign_cast")]
    #[doc(cfg(feature = "integer_sign_cast"))]
    pub const fn cast_signed(self) -> $crate::int<S> {
      panic!("cast_signed")
    }

    pub const fn div_ceil(self, rhs: Self) -> Self {
      panic!("div_ceil")
    }

    #[cfg(feature = "unsigned_is_multiple_of")]
    #[doc(cfg(feature = "unsigned_is_multiple_of"))]
    pub const fn is_multiple_of(self, rhs: Self) -> bool {
      panic!("is_multiple_of")
    }

    pub const fn is_power_of_two(self) -> bool {
      panic!("is_power_of_two")
    }

    pub const fn next_multiple_of(self, rhs: Self) -> Self {
      panic!("next_multiple_of")
    }

    pub const fn next_power_of_two(self) -> Self {
      panic!("next_power_of_two")
    }
  };
  (int) => {
    $crate::macros::generic!(core, int, false);

    pub const fn abs(self) -> Self {
      panic!("abs")
    }

    #[cfg(feature = "integer_sign_cast")]
    #[doc(cfg(feature = "integer_sign_cast"))]
    pub const fn cast_unsigned(self) -> $crate::uint<S> {
      panic!("cast_unsigned")
    }

    #[cfg(feature = "int_roundings")]
    #[doc(cfg(feature = "int_roundings"))]
    pub const fn div_ceil(self, rhs: Self) -> Self {
      panic!("div_ceil")
    }

    pub const fn is_negative(self) -> bool {
      panic!("is_negative")
    }

    pub const fn is_positive(self) -> bool {
      panic!("is_positive")
    }

    #[cfg(feature = "int_roundings")]
    #[doc(cfg(feature = "int_roundings"))]
    pub const fn next_multiple_of(self, rhs: Self) -> Self {
      panic!("next_multiple_of")
    }

    pub const fn signum(self) -> Self {
      panic!("signum")
    }

    pub const fn unsigned_abs(self) -> $crate::uint<S> {
      panic!("unsigned_abs")
    }
  };
}

pub(crate) use generic;
