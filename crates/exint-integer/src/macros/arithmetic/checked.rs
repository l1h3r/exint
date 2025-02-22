macro_rules! checked {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::checked!(core, uint, true);
  };
  (int) => {
    $crate::macros::checked!(core, int, false);
  };
}

pub(crate) use checked;
