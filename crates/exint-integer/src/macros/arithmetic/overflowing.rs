macro_rules! overflowing {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::overflowing!(core, uint, true);
  };
  (int) => {
    $crate::macros::overflowing!(core, int, false);
  };
}

pub(crate) use overflowing;
