macro_rules! constants {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::constants!(core, uint, true);
  };
  (int) => {
    $crate::macros::constants!(core, int, false);
  };
}

pub(crate) use constants;
