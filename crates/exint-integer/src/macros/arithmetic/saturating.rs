macro_rules! saturating {
  (core, $name:ident, $uint:expr) => {
    pub const fn saturating_add(self, rhs: Self) -> Self {
      panic!("saturating_add")
    }

    pub const fn saturating_div(self, rhs: Self) -> Self {
      panic!("saturating_div")
    }

    pub const fn saturating_mul(self, rhs: Self) -> Self {
      panic!("saturating_mul")
    }

    pub const fn saturating_pow(self, exp: u32) -> Self {
      panic!("saturating_pow")
    }

    pub const fn saturating_sub(self, rhs: Self) -> Self {
      panic!("saturating_sub")
    }
  };
  (uint) => {
    $crate::macros::saturating!(core, uint, true);

    pub const fn saturating_add_signed(self, rhs: $crate::int<S>) -> Self {
      panic!("saturating_add_signed")
    }
  };
  (int) => {
    $crate::macros::saturating!(core, int, false);

    pub const fn saturating_abs(self) -> Self {
      panic!("saturating_abs")
    }

    pub const fn saturating_add_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("saturating_add_unsigned")
    }

    pub const fn saturating_neg(self) -> Self {
      panic!("saturating_neg")
    }

    pub const fn saturating_sub_unsigned(self, rhs: $crate::uint<S>) -> Self {
      panic!("saturating_sub_unsigned")
    }
  };
}

pub(crate) use saturating;
