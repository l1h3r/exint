/// Create a generic unsigned integer from a literal expression.
///
/// Valid suffixes are `u{8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|256|512}`.
///
/// [`u32`][crate::primitive::u32] is used by default if no suffix is provided.
///
/// # Examples
///
/// ```
/// use exint::uint;
///
/// const A: uint<1> = uint!(255 u8);
/// assert_eq!(A.to_string(), "255");
///
/// const B: uint<2> = uint!(65535 u16);
/// assert_eq!(B.to_string(), "65535");
///
/// const C: uint<4> = uint!(4294967295 u32);
/// assert_eq!(C.to_string(), "4294967295");
///
/// const D: uint<8> = uint!(18446744073709551615 u64);
/// assert_eq!(D.to_string(), "18446744073709551615");
///
/// const E: uint<16> = uint!(340282366920938463463374607431768211455 u128);
/// assert_eq!(E.to_string(), "340282366920938463463374607431768211455");
/// ```
#[macro_export]
macro_rules! uint {
  ($($tt:tt)+) => {
    $crate::__uint!($($tt)+);
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __uint {
  ($input:literal)      => { $crate::__uint!($input u32) }; // Default to u32 if no suffix
  ($input:literal u8)   => { $crate::__uint!(@parse uint<1> stringify!($input)) };
  ($input:literal u16)  => { $crate::__uint!(@parse uint<2> stringify!($input)) };
  ($input:literal u24)  => { $crate::__uint!(@parse uint<3> stringify!($input)) };
  ($input:literal u32)  => { $crate::__uint!(@parse uint<4> stringify!($input)) };
  ($input:literal u40)  => { $crate::__uint!(@parse uint<5> stringify!($input)) };
  ($input:literal u48)  => { $crate::__uint!(@parse uint<6> stringify!($input)) };
  ($input:literal u56)  => { $crate::__uint!(@parse uint<7> stringify!($input)) };
  ($input:literal u64)  => { $crate::__uint!(@parse uint<8> stringify!($input)) };
  ($input:literal u72)  => { $crate::__uint!(@parse uint<9> stringify!($input)) };
  ($input:literal u80)  => { $crate::__uint!(@parse uint<10> stringify!($input)) };
  ($input:literal u88)  => { $crate::__uint!(@parse uint<11> stringify!($input)) };
  ($input:literal u96)  => { $crate::__uint!(@parse uint<12> stringify!($input)) };
  ($input:literal u104) => { $crate::__uint!(@parse uint<13> stringify!($input)) };
  ($input:literal u112) => { $crate::__uint!(@parse uint<14> stringify!($input)) };
  ($input:literal u120) => { $crate::__uint!(@parse uint<15> stringify!($input)) };
  ($input:literal u128) => { $crate::__uint!(@parse uint<16> stringify!($input)) };
  ($input:literal u256) => { $crate::__uint!(@parse uint<32> stringify!($input)) };
  ($input:literal u512) => { $crate::__uint!(@parse uint<64> stringify!($input)) };
  (@parse uint<$size:literal> $input:expr_2021) => { const { $crate::uint::<$size>::__parse($input) } };
}

/// Create a generic signed integer from a literal expression.
///
/// Valid suffixes are `i{8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|256|512}`.
///
/// [`i32`][crate::primitive::i32] is used by default if no suffix is provided.
///
/// # Examples
///
/// ```
/// use exint::int;
///
/// const A: int<1> = int!(-128 i8);
/// assert_eq!(A.to_string(), "-128");
///
/// const B: int<2> = int!(-32768 i16);
/// assert_eq!(B.to_string(), "-32768");
///
/// const C: int<4> = int!(-2147483648 i32);
/// assert_eq!(C.to_string(), "-2147483648");
///
/// const D: int<8> = int!(-9223372036854775808 i64);
/// assert_eq!(D.to_string(), "-9223372036854775808");
///
/// const E: int<16> = int!(-170141183460469231731687303715884105728 i128);
/// assert_eq!(E.to_string(), "-170141183460469231731687303715884105728");
/// ```
#[macro_export]
macro_rules! int {
  ($($tt:tt)+) => {
    $crate::__int!($($tt)+)
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __int {
  ($input:literal)      => { $crate::__int!($input i32) }; // Default to i32 if no suffix
  ($input:literal i8)   => { $crate::__int!(@parse int<1> stringify!($input)) };
  ($input:literal i16)  => { $crate::__int!(@parse int<2> stringify!($input)) };
  ($input:literal i24)  => { $crate::__int!(@parse int<3> stringify!($input)) };
  ($input:literal i32)  => { $crate::__int!(@parse int<4> stringify!($input)) };
  ($input:literal i40)  => { $crate::__int!(@parse int<5> stringify!($input)) };
  ($input:literal i48)  => { $crate::__int!(@parse int<6> stringify!($input)) };
  ($input:literal i56)  => { $crate::__int!(@parse int<7> stringify!($input)) };
  ($input:literal i64)  => { $crate::__int!(@parse int<8> stringify!($input)) };
  ($input:literal i72)  => { $crate::__int!(@parse int<9> stringify!($input)) };
  ($input:literal i80)  => { $crate::__int!(@parse int<10> stringify!($input)) };
  ($input:literal i88)  => { $crate::__int!(@parse int<11> stringify!($input)) };
  ($input:literal i96)  => { $crate::__int!(@parse int<12> stringify!($input)) };
  ($input:literal i104) => { $crate::__int!(@parse int<13> stringify!($input)) };
  ($input:literal i112) => { $crate::__int!(@parse int<14> stringify!($input)) };
  ($input:literal i120) => { $crate::__int!(@parse int<15> stringify!($input)) };
  ($input:literal i128) => { $crate::__int!(@parse int<16> stringify!($input)) };
  ($input:literal i256) => { $crate::__int!(@parse int<32> stringify!($input)) };
  ($input:literal i512) => { $crate::__int!(@parse int<64> stringify!($input)) };
  (@parse int<$size:literal> $input:expr_2021) => { const { $crate::int::<$size>::__parse($input) } };
}

macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> crate::$name<N> {
      // TODO: Improve this mess and handle same syntax as builtin literals
      // - support underscores
      // - fully support b'<char>'
      #[doc(hidden)]
      #[must_use]
      pub const fn __parse(input: &'static str) -> Self {
        let (bytes, radix): (&'static [u8], u32) = match input.as_bytes() {
          [b'b', b'\'', body @ .., b'\''] => return Self::__parse_char(body),
          [b'0', b'b', tail @ ..] => (tail, 2),
          [b'0', b'o', tail @ ..] => (tail, 8),
          [b'0', b'x', tail @ ..] => (tail, 16),
          bytes => (bytes, 10),
        };

        Self::__parse_text(bytes, radix)
      }

      const fn __parse_char(input: &'static [u8]) -> Self {
        match input {
          [b'b', b'\'', ch, b'\''] => {
            Self::from_u8(*ch)
          }
          [b'b', b'\'', b'\\', b't', b'\''] => {
            Self::from_u8(b'\t')
          }
          [b'b', b'\'', b'\\', b'r', b'\''] => {
            Self::from_u8(b'\r')
          }
          [b'b', b'\'', b'\\', b'n', b'\''] => {
            Self::from_u8(b'\n')
          }
          [b'b', b'\'', b'\\', b'\\', b'\''] => {
            Self::from_u8(b'\\')
          }
          [b'b', b'\'', b'\\', b'\'', b'\''] => {
            Self::from_u8(b'\'')
          }
          [b'b', b'\'', b'\\', 0x00..=0x1F | 0x7F, b'\''] => {
            ::core::panic!("TODO - hex escape")
          }
          [b'b', b'\'', b'\\', _byte, b'\''] => {
            ::core::panic!("TODO - maybe verbatim")
          }
          [b'b', b'\'', b'\\', b'x', body @ .., b'\''] => {
            Self::__parse_text(body, 16)
          }
          _ => {
            ::core::unreachable!()
          }
        }
      }

      const fn __parse_text(input: &'static [u8], radix: u32) -> Self {
        match Self::from_ascii_radix(input, radix) {
          ::core::result::Result::Ok(value) => value,
          ::core::result::Result::Err(error) => ::core::panic!("{}", error.as_str()),
        }
      }
    }
  };
}

implement!(int);
implement!(uint);
