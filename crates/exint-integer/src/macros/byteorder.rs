macro_rules! byteorder {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::byteorder!(core, uint, true);
  };
  (int) => {
    $crate::macros::byteorder!(core, int, false);
  };
}

pub(crate) use byteorder;
