macro_rules! wrapping {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::wrapping!(core, uint, true);
  };
  (int) => {
    $crate::macros::wrapping!(core, int, false);
  };
}

pub(crate) use wrapping;
