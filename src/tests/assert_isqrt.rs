macro_rules! assert_isqrt {
  (isqrt, $name:ident, $value:expr) => {
    $crate::tests::assert_isqrt!(@call isqrt, $name, $value);
  };
  (checked_isqrt, $name:ident, $value:expr) => {
    $crate::tests::assert_isqrt!(@call checked_isqrt, $name, $value, unwrap);
  };
  (checked_isqrt, $_name:ident, $value:expr, None) => {
    $crate::tests::assert_eq!($value.checked_isqrt(), None);
  };
  (@call $method:ident, $name:ident, $value:expr $(, $unwrap:ident)?) => {{
    let this = $value.$method();

    $(
      let this = this.$unwrap();
    )?

    let that = this + T::P_1;
    let out1: Option<bool> = this.checked_mul(this).map(|output| output <= $value);
    let out2: Option<bool> = that.checked_mul(that).map(|output| $value < output);

    $crate::tests::assert_eq!(out1, Some(true), concat!(stringify!($method), "({})"), $value);
    $crate::tests::assert_ne!(out2, Some(false), concat!(stringify!($method), "({})"), $value);
  }};
}

pub(crate) use assert_isqrt;
