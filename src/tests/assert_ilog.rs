macro_rules! assert_ilog {
  (ilog($base:expr), $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call ilog($base), log, $name, $value)
  };
  (ilog2, $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call ilog2, log2, $name, $value)
  };
  (ilog10, $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call ilog10, log10, $name, $value)
  };
  (checked_ilog($base:expr), $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call checked_ilog($base), log, $name, $value, Some)
  };
  (checked_ilog2, $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call checked_ilog2, log2, $name, $value, Some)
  };
  (checked_ilog10, $name:ident, $value:expr) => {
    $crate::tests::assert_ilog!(@call checked_ilog10, log10, $name, $value, Some)
  };
  (@call $ilog:ident $(($base:expr))?, $log:ident, $name:ident, $value:expr $(, $some:ident)?) => {{
    if $name::BITS <= 24 {
      // f32 can repr integers <= 24-bits
      // Note: We use f64 because f32 is often imprecise
      $crate::tests::assert_ilog!([f64, into_u64], $ilog $(($base))?, $log, $value $(, $some)?);
    } else if $name::BITS <= 53 {
      // f64 can repr integers <= 53-bits
      $crate::tests::assert_ilog!([f64, into_u64], $ilog $(($base))?, $log, $value $(, $some)?);
    } else {
      let value = $value % ($name::P_2.pow(53) - $name::P_1);
      $crate::tests::assert_ilog!([f64, into_u64], $ilog $(($base))?, $log, value $(, $some)?);
    }
  }};
  ([$float:ident, $conv:ident], $ilog:ident $(($base:expr))?, $log:ident, $value:expr $(, $some:ident)?) => {{
    fn round(value: $float) -> u32 {
      (value + $float::EPSILON) as u32
    }

    let this = $value.$ilog($($base)?);
    let that: u32 = round(($value.$conv() as $float).$log($($base.into_u8() as $float)?));

    $(
      let that: Option<u32> = $some(that);
    )?

    $crate::tests::assert_eq!(this, that);
  }};
}

pub(crate) use assert_ilog;
