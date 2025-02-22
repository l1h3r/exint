macro_rules! inspect {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::inspect!(core, uint, true);
  };
  (int) => {
    $crate::macros::inspect!(core, int, false);
  };
}

pub(crate) use inspect;
