macro_rules! saturating {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::saturating!(core, uint, true);
  };
  (int) => {
    $crate::macros::saturating!(core, int, false);
  };
}

pub(crate) use saturating;
