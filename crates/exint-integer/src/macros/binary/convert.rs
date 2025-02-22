macro_rules! convert {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::convert!(core, uint, true);
  };
  (int) => {
    $crate::macros::convert!(core, int, false);
  };
}

pub(crate) use convert;
