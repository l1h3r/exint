macro_rules! convert_try {
  ($from:ty, $into:ty, |$binding:ident| $block:block) => {
    impl ::core::convert::TryFrom<$from> for $into {
      type Error = $crate::errors::TryFromIntError;

      #[inline]
      fn try_from($binding: $from) -> Result<Self, Self::Error> {
        $block
      }
    }
  };
}

macro_rules! convert_bool {
  ($name:ident) => {
    impl<const S: usize> ::core::convert::From<bool> for $name<S> {
      #[inline]
      fn from(other: bool) -> Self {
        Self::from_bool(other)
      }
    }
  };
}

macro_rules! convert_cast {
  (@impl From<$from:ty> for $into:ty) => {
    impl ::core::convert::From<$from> for $into {
      #[inline]
      fn from(other: $from) -> Self {
        $crate::macros::cast!($from as $into, other)
      }
    }
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_cast!(@impl From<$std> for uint<$head>);
    $crate::traits::convert_upper_bounded!(@impl From<uint<$head>> for $std);
    $crate::traits::convert_cast!($std -> uint<$($tail)|*>);
  };
  ($std:ident -> int<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_cast!(@impl From<$std> for int<$head>);
    $crate::traits::convert_range_bounded!(@impl From<int<$head>> for $std);
    $crate::traits::convert_cast!($std -> int<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! convert_same {
  ($std:ident -> $lib:ty) => {
    $crate::traits::convert_cast!(@impl From<$std> for $lib);
    $crate::traits::convert_cast!(@impl From<$lib> for $std);
  };
}

macro_rules! convert_lower_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    $crate::traits::convert_try!($from, $into, |other| {
      if other >= <$from as $crate::value::Value>::ZERO {
        Ok($crate::macros::cast!($from as $into, other))
      } else {
        Err($crate::errors::TryFromIntError::new())
      }
    });
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_lower_bounded!(@impl From<$std> for uint<$head>);
    $crate::traits::convert_upper_bounded!(@impl From<uint<$head>> for $std);
    $crate::traits::convert_lower_bounded!($std -> uint<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! convert_upper_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    $crate::traits::convert_try!($from, $into, |other| {
      if other > $crate::macros::cast!($into as $from, <$into>::MAX) {
        Err($crate::errors::TryFromIntError::new())
      } else {
        Ok($crate::macros::cast!($from as $into, other))
      }
    });
  };
  ($std:ident -> uint<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_upper_bounded!(@impl From<$std> for uint<$head>);
    $crate::traits::convert_cast!(@impl From<uint<$head>> for $std);
    $crate::traits::convert_upper_bounded!($std -> uint<$($tail)|*>);
  };
  ($std:ident -> int<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_upper_bounded!(@impl From<$std> for int<$head>);
    $crate::traits::convert_lower_bounded!(@impl From<int<$head>> for $std);
    $crate::traits::convert_upper_bounded!($std -> int<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

macro_rules! convert_range_bounded {
  (@impl From<$from:ty> for $into:ty) => {
    $crate::traits::convert_try!($from, $into, |other| {
      let min: $from = $crate::macros::cast!($into as $from, <$into>::MIN);
      let max: $from = $crate::macros::cast!($into as $from, <$into>::MAX);

      if other < min || other > max {
        Err($crate::errors::TryFromIntError::new())
      } else {
        Ok($crate::macros::cast!($from as $into, other))
      }
    });
  };
  ($std:ident -> $name:ident<$head:literal $(| $tail:literal)*>) => {
    $crate::traits::convert_range_bounded!(@impl From<$std> for $name<$head>);
    $crate::traits::convert_cast!(@impl From<$name<$head>> for $std);
    $crate::traits::convert_range_bounded!($std -> $name<$($tail)|*>);
  };
  ($_std:ident -> $name:ident<>) => {};
}

// TODO: From<uint<S>> for int<S> via convert_upper_bounded
// TODO: From<int<S>> for uint<S> via convert_lower_bounded
// TODO: Platform-specific pointer sizes
macro_rules! convert {
  (uint) => {
    // boolean to unsigned
    $crate::traits::convert_bool!(uint);

    // unsigned to unsigned - same size
    $crate::traits::convert_same!(u8   -> uint<1>);
    $crate::traits::convert_same!(u16  -> uint<2>);
    $crate::traits::convert_same!(u32  -> uint<4>);
    $crate::traits::convert_same!(u64  -> uint<8>);
    $crate::traits::convert_same!(u128 -> uint<16>);

    // unsigned (small) to unsigned (large)
    $crate::traits::convert_cast!(u8   -> uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u16  -> uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u32  -> uint<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u64  -> uint<9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u128 -> uint<32|64>);

    // unsigned (large) to unsigned (small)
    $crate::traits::convert_upper_bounded!(u16  -> uint<1>);
    $crate::traits::convert_upper_bounded!(u32  -> uint<1|2|3>);
    $crate::traits::convert_upper_bounded!(u64  -> uint<1|2|3|4|5|6|7>);
    $crate::traits::convert_upper_bounded!(u128 -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // signed (small) to unsigned (large)
    $crate::traits::convert_lower_bounded!(i8   -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_lower_bounded!(i16  -> uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_lower_bounded!(i32  -> uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_lower_bounded!(i64  -> uint<8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_lower_bounded!(i128 -> uint<16|32|646>);

    // signed (large) to unsigned (small)
    $crate::traits::convert_range_bounded!(i16  -> uint<1>);
    $crate::traits::convert_range_bounded!(i32  -> uint<1|2>);
    $crate::traits::convert_range_bounded!(i64  -> uint<1|2|3|4|5|6|7>);
    $crate::traits::convert_range_bounded!(i128 -> uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // unsigned (pointer) to unsigned (small)
    $crate::traits::convert_upper_bounded!(usize -> uint<1>);

    // signed (pointer) to unsigned (small)
    $crate::traits::convert_range_bounded!(isize -> uint<1>);
  };
  (int) => {
    // boolean to signed
    $crate::traits::convert_bool!(int);

    // signed to signed - same size
    $crate::traits::convert_same!(i8   -> int<1>);
    $crate::traits::convert_same!(i16  -> int<2>);
    $crate::traits::convert_same!(i32  -> int<4>);
    $crate::traits::convert_same!(i64  -> int<8>);
    $crate::traits::convert_same!(i128 -> int<16>);

    // signed (small) to signed (large)
    $crate::traits::convert_cast!(i8   -> int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(i16  -> int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(i32  -> int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(i64  -> int<9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(i128 -> int<32|64>);

    // signed (large) to signed (small)
    $crate::traits::convert_range_bounded!(i16  -> int<1>);
    $crate::traits::convert_range_bounded!(i32  -> int<1|2|3>);
    $crate::traits::convert_range_bounded!(i64  -> int<1|2|3|4|5|6|7>);
    $crate::traits::convert_range_bounded!(i128 -> int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

    // unsigned (small) to signed (large)
    $crate::traits::convert_cast!(u8   -> int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u16  -> int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u32  -> int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u64  -> int<9|10|11|12|13|14|15|16|32|64>);
    $crate::traits::convert_cast!(u128 -> int<32|64>);

    // unsigned (large) to signed (small)
    $crate::traits::convert_upper_bounded!(u8   -> int<1>);
    $crate::traits::convert_upper_bounded!(u16  -> int<1|2>);
    $crate::traits::convert_upper_bounded!(u32  -> int<1|2|3|4>);
    $crate::traits::convert_upper_bounded!(u64  -> int<1|2|3|4|5|6|7|8>);
    $crate::traits::convert_upper_bounded!(u128 -> int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16>);

    // signed (pointer) to signed (small)
    $crate::traits::convert_range_bounded!(isize -> int<1>);

    // unsigned (pointer) to signed (small)
    $crate::traits::convert_upper_bounded!(usize -> int<1>);
  };
}

pub(crate) use convert;
pub(crate) use convert_bool;
pub(crate) use convert_cast;
pub(crate) use convert_lower_bounded;
pub(crate) use convert_range_bounded;
pub(crate) use convert_same;
pub(crate) use convert_try;
pub(crate) use convert_upper_bounded;
