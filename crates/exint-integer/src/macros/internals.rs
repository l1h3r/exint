macro_rules! internals {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::internals!(core, uint, true);
  };
  (int) => {
    $crate::macros::internals!(core, int, false);
  };
}

pub(crate) use internals;
