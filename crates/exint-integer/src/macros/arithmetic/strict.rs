macro_rules! strict {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::strict!(core, uint, true);
  };
  (int) => {
    $crate::macros::strict!(core, int, false);
  };
}

pub(crate) use strict;
