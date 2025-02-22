macro_rules! unbounded {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::unbounded!(core, uint, true);
  };
  (int) => {
    $crate::macros::unbounded!(core, int, false);
  };
}

pub(crate) use unbounded;
