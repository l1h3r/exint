macro_rules! byteorder {
  (core, $name:ident, $uint:expr) => {
    pub const fn from_be(x: Self) -> Self {
      panic!("from_be")
    }

    pub const fn from_be_bytes(bytes: [u8; S]) -> Self {
      panic!("from_be_bytes")
    }

    pub const fn from_le(x: Self) -> Self {
      panic!("from_le")
    }

    pub const fn from_le_bytes(bytes: [u8; S]) -> Self {
      panic!("from_le_bytes")
    }

    pub const fn from_ne_bytes(bytes: [u8; S]) -> Self {
      panic!("from_ne_bytes")
    }

    pub const fn to_be(self) -> Self {
      panic!("to_be")
    }

    pub const fn to_be_bytes(self) -> [u8; S] {
      panic!("to_be_bytes")
    }

    pub const fn to_le(self) -> Self {
      panic!("to_le")
    }

    pub const fn to_le_bytes(self) -> [u8; S] {
      panic!("to_le_bytes")
    }

    pub const fn to_ne_bytes(self) -> [u8; S] {
      panic!("to_ne_bytes")
    }
  };
  (uint) => {
    $crate::macros::byteorder!(core, uint, true);
  };
  (int) => {
    $crate::macros::byteorder!(core, int, false);
  };
}

pub(crate) use byteorder;
