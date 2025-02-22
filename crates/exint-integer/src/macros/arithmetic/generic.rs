macro_rules! generic {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::generic!(core, uint, true);
  };
  (int) => {
    $crate::macros::generic!(core, int, false);
  };
}

pub(crate) use generic;
