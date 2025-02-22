macro_rules! convert {
  (core, $name:ident, $uint:expr) => {
    pub const fn reverse_bits(self) -> Self {
      panic!("reverse_bits")
    }

    pub const fn rotate_left(self, n: u32) -> Self {
      panic!("rotate_left")
    }

    pub const fn rotate_right(self, n: u32) -> Self {
      panic!("rotate_right")
    }

    pub const fn swap_bytes(self) -> Self {
      panic!("swap_bytes")
    }
  };
  (uint) => {
    $crate::macros::convert!(core, uint, true);
  };
  (int) => {
    $crate::macros::convert!(core, int, false);
  };
}

pub(crate) use convert;
