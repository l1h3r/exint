macro_rules! unchecked {
  (core, $name:ident, $uint:expr) => {
    pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
      panic!("unchecked_add")
    }

    pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
      panic!("unchecked_mul")
    }

    #[cfg(feature = "unchecked_shifts")]
    #[doc(cfg(feature = "unchecked_shifts"))]
    pub const unsafe fn unchecked_shl(self, rhs: u32) -> Self {
      panic!("unchecked_shl")
    }

    #[cfg(feature = "unchecked_shifts")]
    #[doc(cfg(feature = "unchecked_shifts"))]
    pub const unsafe fn unchecked_shr(self, rhs: u32) -> Self {
      panic!("unchecked_shr")
    }

    pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
      panic!("unchecked_sub")
    }
  };
  (uint) => {
    $crate::macros::unchecked!(core, uint, true);
  };
  (int) => {
    $crate::macros::unchecked!(core, int, false);

    #[cfg(feature = "unchecked_neg")]
    #[doc(cfg(feature = "unchecked_neg"))]
    pub const unsafe fn unchecked_neg(self) -> Self {
      panic!("unchecked_neg")
    }
  };
}

pub(crate) use unchecked;
