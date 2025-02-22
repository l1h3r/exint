macro_rules! bigint {
  (core, $name:ident, $uint:expr) => {

  };
  (uint) => {
    $crate::macros::bigint!(core, uint, true);
  };
  (int) => {
    $crate::macros::bigint!(core, int, false);
  };
}

pub(crate) use bigint;
