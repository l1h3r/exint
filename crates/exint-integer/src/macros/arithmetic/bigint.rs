macro_rules! bigint {
  (core, $name:ident, $uint:expr) => {
    #[cfg(feature = "bigint_helper_methods")]
    #[doc(cfg(feature = "bigint_helper_methods"))]
    pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
      panic!("borrowing_sub")
    }

    #[cfg(feature = "bigint_helper_methods")]
    #[doc(cfg(feature = "bigint_helper_methods"))]
    pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
      panic!("carrying_add")
    }
  };
  (uint) => {
    $crate::macros::bigint!(core, uint, true);

    #[cfg(feature = "bigint_helper_methods")]
    #[doc(cfg(feature = "bigint_helper_methods"))]
    pub const fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, bool) {
      panic!("carrying_mul")
    }

    #[cfg(feature = "bigint_helper_methods")]
    #[doc(cfg(feature = "bigint_helper_methods"))]
    pub const fn widening_mul(self, rhs: Self) -> (Self, Self) {
      panic!("widening_mul")
    }
  };
  (int) => {
    $crate::macros::bigint!(core, int, false);
  };
}

pub(crate) use bigint;
