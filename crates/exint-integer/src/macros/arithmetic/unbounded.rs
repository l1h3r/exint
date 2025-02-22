macro_rules! unbounded {
  (core, $name:ident, $uint:expr) => {
    #[cfg(feature = "unbounded_shifts")]
    #[doc(cfg(feature = "unbounded_shifts"))]
    pub const fn unbounded_shl(self, rhs: u32) -> Self {
      panic!("unbounded_shl")
    }

    #[cfg(feature = "unbounded_shifts")]
    #[doc(cfg(feature = "unbounded_shifts"))]
    pub const fn unbounded_shr(self, rhs: u32) -> Self {
      panic!("unbounded_shr")
    }
  };
  (uint) => {
    $crate::macros::unbounded!(core, uint, true);
  };
  (int) => {
    $crate::macros::unbounded!(core, int, false);
  };
}

pub(crate) use unbounded;
