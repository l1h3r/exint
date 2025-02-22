macro_rules! unchecked {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::unchecked!(core, uint, true);
  };
  (int) => {
    $crate::macros::unchecked!(core, int, false);
  };
}

pub(crate) use unchecked;
