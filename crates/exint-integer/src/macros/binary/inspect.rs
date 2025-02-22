macro_rules! inspect {
  (core, $name:ident, $uint:expr) => {
    pub const fn count_ones(self) -> u32 {
      panic!("count_ones")
    }

    pub const fn count_zeros(self) -> u32 {
      panic!("count_zeros")
    }

    pub const fn leading_ones(self) -> u32 {
      panic!("leading_ones")
    }

    pub const fn leading_zeros(self) -> u32 {
      panic!("leading_zeros")
    }

    pub const fn trailing_ones(self) -> u32 {
      panic!("trailing_ones")
    }

    pub const fn trailing_zeros(self) -> u32 {
      panic!("trailing_zeros")
    }
  };
  (uint) => {
    $crate::macros::inspect!(core, uint, true);
  };
  (int) => {
    $crate::macros::inspect!(core, int, false);
  };
}

pub(crate) use inspect;
