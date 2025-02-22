use crate::error::TryFromIntError;
use crate::types::int;
use crate::types::uint;
use crate::utils::include_doc;
use crate::utils::Cast;

// -----------------------------------------------------------------------------
// Crate Types
// -----------------------------------------------------------------------------

impl<const N: usize, const M: usize> ::core::convert::TryFrom<int<M>> for uint<N> {
  type Error = TryFromIntError;

  #[inline]
  fn try_from(other: int<M>) -> ::core::result::Result<Self, Self::Error> {
    if other < Cast::cast(Self::MIN) || other > Cast::cast(Self::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(Cast::cast(other))
    }
  }
}

impl<const N: usize, const M: usize> ::core::convert::TryFrom<uint<M>> for int<N> {
  type Error = TryFromIntError;

  #[inline]
  fn try_from(other: uint<M>) -> ::core::result::Result<Self, Self::Error> {
    if other < Cast::cast(Self::MIN) || other > Cast::cast(Self::MAX) {
      ::core::result::Result::Err(TryFromIntError::new())
    } else {
      ::core::result::Result::Ok(Cast::cast(other))
    }
  }
}

// -----------------------------------------------------------------------------
// ASCII Character
// -----------------------------------------------------------------------------

#[cfg(feature = "ascii_char")]
impl<const N: usize> ::core::convert::From<::core::ascii::Char> for uint<N> {
  #[inline]
  fn from(other: ::core::ascii::Char) -> uint<N> {
    Self::from_u8(other as u8)
  }
}

// -----------------------------------------------------------------------------
// Boolean
// -----------------------------------------------------------------------------

impl<const N: usize> ::core::convert::From<bool> for uint<N> {
  #[doc = include_doc!(uint, "convert_from_bool")]
  #[inline]
  fn from(other: bool) -> Self {
    Self::from_bool(other)
  }
}

impl<const N: usize> ::core::convert::From<bool> for int<N> {
  #[doc = include_doc!(int, "convert_from_bool")]
  #[inline]
  fn from(other: bool) -> Self {
    Self::from_bool(other)
  }
}

// -----------------------------------------------------------------------------
// Unicode Scalar Value (char)
// -----------------------------------------------------------------------------

impl ::core::convert::TryFrom<char> for uint<1> {
  type Error = ::core::char::TryFromCharError;

  /// Tries to convert a [`char`] into a [`uint<1>`].
  #[inline]
  fn try_from(other: char) -> ::core::result::Result<Self, Self::Error> {
    u8::try_from(other).map(Self::from_u8)
  }
}

impl ::core::convert::TryFrom<char> for uint<2> {
  type Error = ::core::char::TryFromCharError;

  /// Tries to convert a [`char`] into a [`uint<2>`].
  #[inline]
  fn try_from(other: char) -> ::core::result::Result<Self, Self::Error> {
    u16::try_from(other).map(Self::from_u16)
  }
}

impl ::core::convert::TryFrom<char> for uint<3> {
  type Error = ::core::char::TryFromCharError;

  /// Tries to convert a [`char`] into a [`uint<3>`].
  #[inline]
  fn try_from(_other: char) -> ::core::result::Result<Self, Self::Error> {
    ::core::todo!("TryFrom<char> for u24")
  }
}

macro_rules! implement_from_char {
  ($name:ident<$size:literal>, $conv:ident) => {
    impl ::core::convert::From<char> for $name<$size> {
      #[doc = concat!("Converts a [`char`] into a [`", stringify!($name), "<", $size, ">`].")]
      #[inline]
      fn from(other: char) -> Self {
        Self::$conv(::core::convert::Into::into(other))
      }
    }
  };
}

implement_from_char!(uint<4>,  from_u32);
implement_from_char!(uint<5>,  from_u64);
implement_from_char!(uint<6>,  from_u64);
implement_from_char!(uint<7>,  from_u64);
implement_from_char!(uint<8>,  from_u64);
implement_from_char!(uint<9>,  from_u128);
implement_from_char!(uint<10>, from_u128);
implement_from_char!(uint<11>, from_u128);
implement_from_char!(uint<12>, from_u128);
implement_from_char!(uint<13>, from_u128);
implement_from_char!(uint<14>, from_u128);
implement_from_char!(uint<15>, from_u128);
implement_from_char!(uint<16>, from_u128);

impl ::core::convert::From<uint<1>> for char {
  #[doc = include_doc!(uint, "convert_u8_char")]
  #[inline]
  fn from(other: uint<1>) -> Self {
    ::core::convert::Into::into(other.into_u8())
  }
}

impl ::core::convert::TryFrom<uint<4>> for char {
  type Error = ::core::char::CharTryFromError;

  #[inline]
  fn try_from(other: uint<4>) -> ::core::result::Result<Self, Self::Error> {
    ::core::convert::TryInto::try_into(other.into_u32())
  }
}

// -----------------------------------------------------------------------------
// IPv4/IPv6 Address
// -----------------------------------------------------------------------------

impl ::core::convert::From<uint<4>> for ::core::net::Ipv4Addr {
  /// Uses [`Ipv4Addr::from_bits`] to convert a host byte order `uint<4>` to an IPv4 address.
  ///
  /// [`Ipv4Addr::from_bits`]: ::core::net::Ipv4Addr::from_bits
  #[inline]
  fn from(other: uint<4>) -> Self {
    Self::from_bits(other.into_u32())
  }
}

impl ::core::convert::From<::core::net::Ipv4Addr> for uint<4> {
  /// Uses [`Ipv4Addr::to_bits`] to convert an IPv4 address to a host byte order `uint<4>`.
  ///
  /// [`Ipv4Addr::to_bits`]: ::core::net::Ipv4Addr::to_bits
  #[inline]
  fn from(other: ::core::net::Ipv4Addr) -> Self {
    Self::from_u32(other.to_bits())
  }
}

impl ::core::convert::From<uint<16>> for ::core::net::Ipv6Addr {
  /// Uses [`Ipv6Addr::from_bits`] to convert a host byte order `uint<16>` to an IPv6 address.
  ///
  /// [`Ipv6Addr::from_bits`]: ::core::net::Ipv6Addr::from_bits
  #[inline]
  fn from(other: uint<16>) -> Self {
    Self::from_bits(other.into_u128())
  }
}

impl ::core::convert::From<::core::net::Ipv6Addr> for uint<16> {
  /// Uses [`Ipv6Addr::to_bits`] to convert an IPv6 address to a host byte order `uint<16>`.
  ///
  /// [`Ipv6Addr::to_bits`]: ::core::net::Ipv6Addr::to_bits
  #[inline]
  fn from(other: ::core::net::Ipv6Addr) -> Self {
    Self::from_u128(other.to_bits())
  }
}

// Not possible due to orphan rules:

// impl From<[uint<1>; 4]> for Ipv4Addr {}
// impl From<[uint<1>; 4]> for IpAddr {}

// impl From<[uint<1>; 16]> for Ipv6Addr {}
// impl From<[uint<1>; 16]> for IpAddr {}

// impl From<[uint<2>; 8]> for Ipv6Addr {}
// impl From<[uint<2>; 8]> for IpAddr {}

// -----------------------------------------------------------------------------
// Socket Address
// -----------------------------------------------------------------------------

// Not possible due to orphan rules:

// impl<I: Into<IpAddr>> From<(I, uint<2>)> for SocketAddr {}

// -----------------------------------------------------------------------------
// Array
// -----------------------------------------------------------------------------

// Not possible due to orphan rules:

// impl<const N: usize> From<[uint<1>; N]> for [u8; N] {}
// impl<const N: usize> From<[u8; N]> for [uint<1>; N] {}

// impl<const N: usize> From<[uint<2>; N]> for [u8; N] {}
// impl<const N: usize> From<[u8; N]> for [uint<2>; N] {}

// impl<const N: usize> From<[uint<4>; N]> for [u8; N] {}
// impl<const N: usize> From<[u8; N]> for [uint<4>; N] {}

// impl<const N: usize> From<[uint<8>; N]> for [u8; N] {}
// impl<const N: usize> From<[u8; N]> for [uint<8>; N] {}

// impl<const N: usize> From<[uint<16>; N]> for [u8; N] {}
// impl<const N: usize> From<[u8; N]> for [uint<16>; N] {}

// -----------------------------------------------------------------------------
// Atomic Integer
// -----------------------------------------------------------------------------

macro_rules! implement_atomic {
  (
    $name:ident<$size:literal>,
    $conv:ident,
    $atomic:ident,
    $meta:meta,
    $doc:expr_2021
  ) => {
    #[$meta]
    impl ::core::convert::From<$name<$size>> for ::core::sync::atomic::$atomic {
      #[doc = $doc]
      #[inline]
      fn from(other: $name<$size>) -> Self {
        Self::new(other.$conv())
      }
    }
  };
}

implement_atomic! {
  uint<1>,
  into_u8,
  AtomicU8,
  cfg(target_has_atomic = "8"),
  include_doc!(uint, "convert_u8_atomic")
}

implement_atomic! {
  uint<2>,
  into_u16,
  AtomicU16,
  cfg(target_has_atomic = "16"),
  include_doc!(uint, "convert_u16_atomic")
}

implement_atomic! {
  uint<4>,
  into_u32,
  AtomicU32,
  cfg(target_has_atomic = "32"),
  include_doc!(uint, "convert_u32_atomic")
}

implement_atomic! {
  uint<8>,
  into_u64,
  AtomicU64,
  cfg(target_has_atomic = "64"),
  include_doc!(uint, "convert_u64_atomic")
}

implement_atomic! {
  uint<16>,
  into_u128,
  AtomicU128,
  cfg(all(target_has_atomic = "128", feature = "integer_atomics")),
  include_doc!(uint, "convert_u128_atomic")
}

implement_atomic! {
  int<1>,
  into_i8,
  AtomicI8,
  cfg(target_has_atomic = "8"),
  include_doc!(int, "convert_i8_atomic")
}

implement_atomic! {
  int<2>,
  into_i16,
  AtomicI16,
  cfg(target_has_atomic = "16"),
  include_doc!(int, "convert_i16_atomic")
}

implement_atomic! {
  int<4>,
  into_i32,
  AtomicI32,
  cfg(target_has_atomic = "32"),
  include_doc!(int, "convert_i32_atomic")
}

implement_atomic! {
  int<8>,
  into_i64,
  AtomicI64,
  cfg(target_has_atomic = "64"),
  include_doc!(int, "convert_i64_atomic")
}

implement_atomic! {
  int<16>,
  into_i128,
  AtomicI128,
  cfg(all(target_has_atomic = "128", feature = "integer_atomics")),
  include_doc!(int, "convert_i128_atomic")
}

// -----------------------------------------------------------------------------
// NonZero
// -----------------------------------------------------------------------------

macro_rules! implement_nonzero {
  (
    $name:ident<$size:literal>,
    $conv_from:ident,
    $conv_into:ident,
    $nonzero:ident,
    $doc:expr_2021
  ) => {
    impl ::core::convert::From<::core::num::$nonzero> for $name<$size> {
      #[inline]
      fn from(other: ::core::num::$nonzero) -> Self {
        Self::$conv_from(other.get())
      }
    }

    impl ::core::convert::TryFrom<$name<$size>> for ::core::num::$nonzero {
      type Error = TryFromIntError;

      #[doc = $doc]
      #[inline]
      fn try_from(other: $name<$size>) -> ::core::result::Result<Self, Self::Error> {
        ::core::num::$nonzero::new(other.$conv_into()).ok_or_else(TryFromIntError::new)
      }
    }
  };
}

implement_nonzero! {
  uint<1>,
  from_u8,
  into_u8,
  NonZeroU8,
  include_doc!(uint, "convert_u8_nonzero")
}

implement_nonzero! {
  uint<2>,
  from_u16,
  into_u16,
  NonZeroU16,
  include_doc!(uint, "convert_u16_nonzero")
}

implement_nonzero! {
  uint<4>,
  from_u32,
  into_u32,
  NonZeroU32,
  include_doc!(uint, "convert_u32_nonzero")
}

implement_nonzero! {
  uint<8>,
  from_u64,
  into_u64,
  NonZeroU64,
  include_doc!(uint, "convert_u64_nonzero")
}

implement_nonzero! {
  uint<16>,
  from_u128,
  into_u128,
  NonZeroU128,
  include_doc!(uint, "convert_u128_nonzero")
}

implement_nonzero! {
  int<1>,
  from_i8,
  into_i8,
  NonZeroI8,
  include_doc!(int, "convert_i8_nonzero")
}

implement_nonzero! {
  int<2>,
  from_i16,
  into_i16,
  NonZeroI16,
  include_doc!(int, "convert_i16_nonzero")
}

implement_nonzero! {
  int<4>,
  from_i32,
  into_i32,
  NonZeroI32,
  include_doc!(int, "convert_i32_nonzero")
}

implement_nonzero! {
  int<8>,
  from_i64,
  into_i64,
  NonZeroI64,
  include_doc!(int, "convert_i64_nonzero")
}

implement_nonzero! {
  int<16>,
  from_i128,
  into_i128,
  NonZeroI128,
  include_doc!(int, "convert_i128_nonzero")
}

// -----------------------------------------------------------------------------
// ExitCode
// -----------------------------------------------------------------------------

#[cfg(feature = "std")]
impl ::core::convert::From<uint<1>> for ::std::process::ExitCode {
  /// Constructs an `ExitCode` from an arbitrary uint<1> value.
  #[inline]
  fn from(other: uint<1>) -> Self {
    ::core::convert::Into::into(other.into_u8())
  }
}

// -----------------------------------------------------------------------------
// Crate Types - Specialization
// -----------------------------------------------------------------------------

macro_rules! docstring {
  ($from:ty, $name:ident<$size:literal>) => {
    concat!("Converts [`", stringify!($from), "`] to [`", stringify!($name), "<", $size, ">`] losslessly.")
  };
  ($name:ident<$size:literal>, $into:ty) => {
    concat!("Converts [`", stringify!($name), "<", $size, ">`] to [`", stringify!($into), "`] losslessly.")
  };
  ($from:ty, $into:ty) => {
    concat!("Converts [`", stringify!($from), "`] to [`", stringify!($into), "`] losslessly.")
  };
}

macro_rules! impl_unbounded {
  (@from, $from:ty, $name:ident<$size:literal>) => {
    impl_unbounded!(@from, docstring!($from, $name<$size>), $from, $name<$size>);
  };
  (@from, $name:ident<$size:literal>, $into:ty) => {
    impl_unbounded!(@from, docstring!($name<$size>, $into), $name<$size>, $into);
  };
  (@from, $from:ty, $into:ty) => {
    impl_unbounded!(@from, docstring!($from, $into), $from, $into);
  };
  (@from, $doc:expr_2021, $from:ty, $into:ty) => {
    impl ::core::convert::From<$from> for $into {
      #[doc = $doc]
      #[inline]
      fn from(other: $from) -> Self {
        $crate::utils::Cast::cast(other)
      }
    }
  };
}

macro_rules! impl_bounded {
  ($from:ty, $into:ty, |$binding:ident| $block:block) => {
    impl ::core::convert::TryFrom<$from> for $into {
      type Error = $crate::error::TryFromIntError;

      /// Tries to create the target number type from a source number type.
      /// This returns an error if the source value is outside of the range of
      /// the target type.
      #[inline]
      fn try_from($binding: $from) -> ::core::result::Result<Self, Self::Error> {
        $block
      }
    }
  };
  (@lower, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      if other >= $crate::utils::Zero::ZERO {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      } else {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      }
    });
  };
  (@upper, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      if other > $crate::utils::Cast::cast(Self::MAX) {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      } else {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      }
    });
  };
  (@range, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      let max: $from = $crate::utils::Cast::cast(Self::MAX);
      let min: $from = $crate::utils::Cast::cast(Self::MIN);

      if other < min || other > max {
        ::core::result::Result::Err($crate::error::TryFromIntError::new())
      } else {
        ::core::result::Result::Ok($crate::utils::Cast::cast(other))
      }
    });
  };
  (@none, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      ::core::result::Result::Ok($crate::utils::Cast::cast(other))
    });
  };
}

macro_rules! impl_float {
  (
    $name:ident<$size:literal> as $integer:ty => $float:ty
    $(where $meta:meta)?
  ) => {
    $(#[$meta])?
    impl ::core::convert::From<$name<$size>> for $float {
      #[doc = docstring!($name<$size>, $float)]
      #[inline]
      fn from(other: $name<$size>) -> Self {
        <$integer>::from(other) as $float
      }
    }
  };
  ($name:ident<$size:literal> as $integer:ty => $($float:ty)+) => {
    $(
      impl_float!($name<$size> as $integer => $float);
    )+
  };
}

macro_rules! impl_same {
  (usize => $lib:ty) => {
    impl_bounded!(@none, usize, $lib);
    impl_bounded!(@none, $lib, usize);
  };
  (isize => $lib:ty) => {
    impl_bounded!(@none, isize, $lib);
    impl_bounded!(@none, $lib, isize);
  };
  ($std:ty => $lib:ty) => {
    impl_unbounded!(@from, $std, $lib);
    impl_unbounded!(@from, $lib, $std);
  };
}

macro_rules! impl_from {
  (usize => uint<$($size:literal)|+>) => {
    $(
      impl_bounded!(@none, usize, uint<$size>);
      impl_bounded!(@upper, uint<$size>, usize);
    )+
  };
  (usize => int<$($size:literal)|+>) => {
    $(
      impl_bounded!(@none, usize, int<$size>);
      impl_bounded!(@range, int<$size>, usize);
    )+
  };
  (isize => uint<$($size:literal)|+>) => {
    compile_error!("FIXME")
  };
  (isize => int<$($size:literal)|+>) => {
    $(
      impl_bounded!(@none, isize, int<$size>);
      impl_bounded!(@range, int<$size>, isize);
    )+
  };
  ($std:ty => uint<$($size:literal)|+>) => {
    $(
      impl_unbounded!(@from, $std, uint<$size>);
      impl_bounded!(@upper, uint<$size>, $std);
    )+
  };
  ($std:ty => int<$($size:literal)|+>) => {
    $(
      impl_unbounded!(@from, $std, int<$size>);
      impl_bounded!(@range, int<$size>, $std);
    )+
  };
}

macro_rules! impl_lower_bounded {
  ($std:ty => uint<$($size:literal)|+>) => {
    $(
      impl_bounded!(@lower, $std, uint<$size>);
      impl_bounded!(@upper, uint<$size>, $std);
    )+
  };
}

macro_rules! impl_upper_bounded {
  (usize => uint<$($size:literal)|+>) => {
    $(
      impl_bounded!(@upper, usize, uint<$size>);
      impl_bounded!(@none, uint<$size>, usize);
    )+
  };
  ($std:ty => uint<$($size:literal)|+>) => {
    $(
      impl_bounded!(@upper, $std, uint<$size>);
      impl_unbounded!(@from, uint<$size>, $std);
    )+
  };
  ($std:ty => int<$($size:literal)|+>) => {
    $(
      impl_bounded!(@upper, $std, int<$size>);
      impl_bounded!(@lower, int<$size>, $std);
    )+
  };
}

macro_rules! impl_range_bounded {
  (isize => $name:ident<$($size:literal)|+>) => {
    $(
      impl_bounded!(@range, isize, $name<$size>);
      impl_bounded!(@none, $name<$size>, isize);
    )+
  };
  ($std:ty => $name:ident<$($size:literal)|+>) => {
    $(
      impl_bounded!(@range, $std, $name<$size>);
      impl_unbounded!(@from, $name<$size>, $std);
    )+
  };
}

// unsigned -> unsigned - same size
impl_same!(u8   => uint<1>);
impl_same!(u16  => uint<2>);
impl_same!(u32  => uint<4>);
impl_same!(u64  => uint<8>);
impl_same!(u128 => uint<16>);

// unsigned -> unsigned - (small -> large)
impl_from!(u8   => uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u16  => uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u32  => uint<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u64  => uint<9|10|11|12|13|14|15|16|32|64>);
impl_from!(u128 => uint<32|64>);

// unsigned -> unsigned - (large -> small)
impl_upper_bounded!(u16  => uint<1>);
impl_upper_bounded!(u32  => uint<1|2|3>);
impl_upper_bounded!(u64  => uint<1|2|3|4|5|6|7>);
impl_upper_bounded!(u128 => uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

// unsigned -> signed - (small -> large)
impl_from!(u8   => int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u16  => int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u32  => int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(u64  => int<9|10|11|12|13|14|15|16|32|64>);
impl_from!(u128 => int<32|64>);

// unsigned -> signed - (large -> small)
impl_upper_bounded!(u8   => int<1>);
impl_upper_bounded!(u16  => int<1|2>);
impl_upper_bounded!(u32  => int<1|2|3|4>);
impl_upper_bounded!(u64  => int<1|2|3|4|5|6|7|8>);
impl_upper_bounded!(u128 => int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16>);

// unsigned -> float
impl_float!(uint<1> as u8  => f32 f64);
impl_float!(uint<2> as u16 => f32 f64);
impl_float!(uint<3> as u32 => f32 f64);
impl_float!(uint<4> as u32 => f64);
impl_float!(uint<5> as u64 => f64);
impl_float!(uint<6> as u64 => f64);

impl_float!(uint<1> as u8 => f16 where cfg(feature = "f16"));

impl_float!(uint<1>  as u8   => f128 where cfg(feature = "f128"));
impl_float!(uint<2>  as u16  => f128 where cfg(feature = "f128"));
impl_float!(uint<3>  as u32  => f128 where cfg(feature = "f128"));
impl_float!(uint<4>  as u32  => f128 where cfg(feature = "f128"));
impl_float!(uint<5>  as u64  => f128 where cfg(feature = "f128"));
impl_float!(uint<6>  as u64  => f128 where cfg(feature = "f128"));
impl_float!(uint<7>  as u64  => f128 where cfg(feature = "f128"));
impl_float!(uint<8>  as u64  => f128 where cfg(feature = "f128"));
impl_float!(uint<9>  as u128 => f128 where cfg(feature = "f128"));
impl_float!(uint<10> as u128 => f128 where cfg(feature = "f128"));
impl_float!(uint<11> as u128 => f128 where cfg(feature = "f128"));
impl_float!(uint<12> as u128 => f128 where cfg(feature = "f128"));
impl_float!(uint<13> as u128 => f128 where cfg(feature = "f128"));
impl_float!(uint<14> as u128 => f128 where cfg(feature = "f128"));

// signed -> signed - same size
impl_same!(i8   => int<1>);
impl_same!(i16  => int<2>);
impl_same!(i32  => int<4>);
impl_same!(i64  => int<8>);
impl_same!(i128 => int<16>);

// signed -> signed - (small -> large)
impl_from!(i8   => int<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(i16  => int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(i32  => int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_from!(i64  => int<9|10|11|12|13|14|15|16|32|64>);
impl_from!(i128 => int<32|64>);

// signed -> signed - (large -> small)
impl_range_bounded!(i16  => int<1>);
impl_range_bounded!(i32  => int<1|2|3>);
impl_range_bounded!(i64  => int<1|2|3|4|5|6|7>);
impl_range_bounded!(i128 => int<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

// signed -> unsigned - (small -> large)
impl_lower_bounded!(i8   => uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_lower_bounded!(i16  => uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_lower_bounded!(i32  => uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
impl_lower_bounded!(i64  => uint<8|9|10|11|12|13|14|15|16|32|64>);
impl_lower_bounded!(i128 => uint<16|32|64>);

// signed -> unsigned - (large -> small)
impl_range_bounded!(i16  => uint<1>);
impl_range_bounded!(i32  => uint<1|2>);
impl_range_bounded!(i64  => uint<1|2|3|4|5|6|7>);
impl_range_bounded!(i128 => uint<1|2|3|4|5|6|7|8|9|10|11|12|13|14|15>);

// signed -> float
impl_float!(int<1> as i8  => f32 f64);
impl_float!(int<2> as i16 => f32 f64);
impl_float!(int<3> as i32 => f32 f64);
impl_float!(int<4> as i32 => f64);
impl_float!(int<5> as i64 => f64);
impl_float!(int<6> as i64 => f64);

impl_float!(int<1> as i8 => f16 where cfg(feature = "f16"));

impl_float!(int<1>  as i8   => f128 where cfg(feature = "f128"));
impl_float!(int<2>  as i16  => f128 where cfg(feature = "f128"));
impl_float!(int<3>  as i32  => f128 where cfg(feature = "f128"));
impl_float!(int<4>  as i32  => f128 where cfg(feature = "f128"));
impl_float!(int<5>  as i64  => f128 where cfg(feature = "f128"));
impl_float!(int<6>  as i64  => f128 where cfg(feature = "f128"));
impl_float!(int<7>  as i64  => f128 where cfg(feature = "f128"));
impl_float!(int<8>  as i64  => f128 where cfg(feature = "f128"));
impl_float!(int<9>  as i128 => f128 where cfg(feature = "f128"));
impl_float!(int<10> as i128 => f128 where cfg(feature = "f128"));
impl_float!(int<11> as i128 => f128 where cfg(feature = "f128"));
impl_float!(int<12> as i128 => f128 where cfg(feature = "f128"));
impl_float!(int<13> as i128 => f128 where cfg(feature = "f128"));
impl_float!(int<14> as i128 => f128 where cfg(feature = "f128"));

// pointer -> {un}signed
#[cfg(target_pointer_width = "16")]
mod platform {
  use super::*;

  impl_same!(usize => uint<2>);
  impl_from!(usize => uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => uint<1>);
  impl_from!(usize => int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => int<1|2>);
  impl_same!(isize => int<2>);
  impl_from!(isize => int<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => int<1>);
  impl_lower_bounded!(isize => uint<2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => uint<1>);
}

#[cfg(target_pointer_width = "32")]
mod platform {
  use super::*;

  impl_same!(usize => uint<4>);
  impl_from!(usize => uint<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => uint<1|2|3>);
  impl_from!(usize => int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => int<1|2|3|4>);
  impl_same!(isize => int<4>);
  impl_from!(isize => int<5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => int<1|2|3>);
  impl_lower_bounded!(isize => uint<3|4|5|6|7|8|9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => uint<1|2>);
}

#[cfg(target_pointer_width = "64")]
mod platform {
  use super::*;

  impl_same!(usize => uint<8>);
  impl_from!(usize => uint<9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => uint<1|2|3|4|5|6|7>);
  impl_from!(usize => int<9|10|11|12|13|14|15|16|32|64>);
  impl_upper_bounded!(usize => int<1|2|3|4|5|6|7|8>);
  impl_same!(isize => int<8>);
  impl_from!(isize => int<9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => int<1|2|3|4|5|6|7>);
  impl_lower_bounded!(isize => uint<8|9|10|11|12|13|14|15|16|32|64>);
  impl_range_bounded!(isize => uint<1|2|3|4|5|6|7>);
}
