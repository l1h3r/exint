macro_rules! parse_str {
  (core, $name:ident, $uint:expr) => {
    pub const fn from_str_radix(src: &str, radix: u32) -> Result<Self, $crate::ParseIntError> {
      panic!("from_str_radix")
    }
  };
  (uint) => {
    $crate::macros::parse_str!(core, uint, true);
  };
  (int) => {
    $crate::macros::parse_str!(core, int, false);
  };
}

pub(crate) use parse_str;
