macro_rules! wrapping {
  (core, $name:ident, $uint:expr) => {
    pub const fn wrapping_add(self, rhs: Self) -> Self {
      panic!("wrapping_add")
    }

    pub const fn wrapping_div(self, rhs: Self) -> Self {
      panic!("wrapping_div")
    }

    pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
      panic!("wrapping_div_euclid")
    }

    pub const fn wrapping_mul(self, rhs: Self) -> Self {
      panic!("wrapping_mul")
    }

    pub const fn wrapping_neg(self) -> Self {
      panic!("wrapping_neg")
    }

    pub const fn wrapping_pow(self, exp: u32) -> Self {
      panic!("wrapping_pow")
    }

    pub const fn wrapping_rem(self, rhs: Self) -> Self {
      panic!("wrapping_rem")
    }

    pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
      panic!("wrapping_rem_euclid")
    }

    pub const fn wrapping_shl(self, rhs: u32) -> Self {
      panic!("wrapping_shl")
    }

    pub const fn wrapping_shr(self, rhs: u32) -> Self {
      panic!("wrapping_shr")
    }

    pub const fn wrapping_sub(self, rhs: Self) -> Self {
      panic!("wrapping_sub")
    }
  };
  (uint) => {
    $crate::macros::wrapping!(core, uint, true);

    pub const fn wrapping_add_signed(self, rhs: $crate::int<S>) -> Self {
      panic!("wrapping_add_signed")
    }

    #[cfg(feature = "wrapping_next_power_of_two")]
    #[doc(cfg(feature = "wrapping_next_power_of_two"))]
    pub const fn wrapping_next_power_of_two(self) -> Self {
      panic!("wrapping_next_power_of_two")
    }
  };
  (int) => {
    $crate::macros::wrapping!(core, int, false);

    pub const fn wrapping_abs(self) -> Self {
      panic!("wrapping_abs")
    }

    pub const fn wrapping_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("wrapping_add_unsigned")
    }

    pub const fn wrapping_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("wrapping_sub_unsigned")
    }
  };
}

pub(crate) use wrapping;
