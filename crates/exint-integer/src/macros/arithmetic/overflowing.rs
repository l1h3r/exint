macro_rules! overflowing {
  (core, $name:ident, $uint:expr) => {
    pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_add")
    }

    pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_div")
    }

    pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_div_euclid")
    }

    pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_mul")
    }

    pub const fn overflowing_neg(self) -> (Self, bool) {
      panic!("overflowing_neg")
    }

    pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
      panic!("overflowing_pow")
    }

    pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_rem")
    }

    pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_rem_euclid")
    }

    pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
      panic!("overflowing_shl")
    }

    pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
      panic!("overflowing_shr")
    }

    pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
      panic!("overflowing_sub")
    }
  };
  (uint) => {
    $crate::macros::overflowing!(core, uint, true);

    pub const fn overflowing_add_signed(self, rhs: $crate::int<S>) -> (Self, bool) {
      panic!("overflowing_add_signed")
    }
  };
  (int) => {
    $crate::macros::overflowing!(core, int, false);

    pub const fn overflowing_abs(self) -> (Self, bool) {
      panic!("overflowing_abs")
    }

    pub const fn overflowing_add_unsigned(self, rhs: $crate::uint<S>) -> (Self, bool) {
      panic!("overflowing_add_unsigned")
    }

    pub const fn overflowing_sub_unsigned(self, rhs: $crate::uint<S>) -> (Self, bool) {
      panic!("overflowing_sub_unsigned")
    }
  };
}

pub(crate) use overflowing;
