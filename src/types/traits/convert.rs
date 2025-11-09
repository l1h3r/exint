use ::core::char::CharTryFromError;
use ::core::convert::From;
use ::core::convert::TryFrom;
use ::core::net::Ipv4Addr;
use ::core::net::Ipv6Addr;
use ::core::result::Result;
use ::core::result::Result::Err;
use ::core::result::Result::Ok;

#[cfg(feature = "ascii_char")]
use ::core::ascii::Char;

#[cfg(feature = "std")]
use ::std::process::ExitCode;

use crate::error::TryFromCharError;
use crate::error::TryFromIntError;
use crate::types::int;
use crate::types::uint;
use crate::utils::Cast;

// -----------------------------------------------------------------------------
// ASCII Character
// -----------------------------------------------------------------------------

const_trait_if! {
  #[feature("const_convert")]
  #[cfg(feature = "ascii_char")]
  impl<const N: usize> const From<Char> for uint<N> {
    #[inline]
    fn from(other: Char) -> uint<N> {
      Self::from_u8(other as u8)
    }
  }
}

// -----------------------------------------------------------------------------
// Boolean
// -----------------------------------------------------------------------------

const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize> const From<bool> for uint<N> {
    #[doc = include_doc!(uint, "convert_from_bool")]
    #[inline]
    fn from(other: bool) -> Self {
      Self::from_bool(other)
    }
  }

  #[feature("const_convert")]
  impl<const N: usize> const From<bool> for int<N> {
    #[doc = include_doc!(int, "convert_from_bool")]
    #[inline]
    fn from(other: bool) -> Self {
      Self::from_bool(other)
    }
  }
}

// -----------------------------------------------------------------------------
// Unicode Scalar Value (char)
// -----------------------------------------------------------------------------

macro_rules! implement_try_from_char {
  ($name:ident<$($size:literal)|+>) => {
    const_trait_if! {
      $(
        #[feature("const_convert")]
        impl const TryFrom<char> for $name<$size> {
          type Error = TryFromCharError;

          #[doc = include_doc!($name, "convert_try_from_char")]
          #[inline]
          fn try_from(other: char) -> Result<Self, Self::Error> {
            match Self::try_from(u32::from(other)) {
              Ok(value) => Ok(value),
              Err(_) => Err(TryFromCharError::new()),
            }
          }
        }
      )+
    }
  };
}

macro_rules! implement_from_char {
  ($name:ident<$($size:literal)|+>) => {
    const_trait_if! {
      $(
        #[feature("const_convert")]
        impl const From<char> for $name<$size> {
          #[doc = include_doc!($name, "convert_from_char")]
          #[inline]
          fn from(other: char) -> Self {
            Self::from_u32(other as u32)
          }
        }
      )+
    }
  };
}

implement_try_from_char!(uint<1|2|3>);
implement_from_char!(uint<4|5|6|7|8|9|10|11|12|13|14|15|16>);

const_trait_if! {
  #[feature("const_convert")]
  impl const From<uint<1>> for char {
    #[doc = include_doc!(uint, "convert_u8_char")]
    #[inline]
    fn from(other: uint<1>) -> Self {
      other.into_u8() as Self
    }
  }

  #[feature("const_convert")]
  impl const TryFrom<uint<4>> for char {
    type Error = CharTryFromError;

    #[inline]
    fn try_from(other: uint<4>) -> Result<Self, Self::Error> {
      Self::try_from(other.into_u32())
    }
  }
}

// -----------------------------------------------------------------------------
// IPv4/IPv6 Address
// -----------------------------------------------------------------------------

const_trait_if! {
  #[feature("const_convert")]
  impl const From<uint<4>> for Ipv4Addr {
    /// Uses [`Ipv4Addr::from_bits`] to convert a host byte order `uint<4>` to an IPv4 address.
    ///
    /// [`Ipv4Addr::from_bits`]: Ipv4Addr::from_bits
    #[inline]
    fn from(other: uint<4>) -> Self {
      Self::from_bits(other.into_u32())
    }
  }

  #[feature("const_convert")]
  impl const From<Ipv4Addr> for uint<4> {
    /// Uses [`Ipv4Addr::to_bits`] to convert an IPv4 address to a host byte order `uint<4>`.
    ///
    /// [`Ipv4Addr::to_bits`]: Ipv4Addr::to_bits
    #[inline]
    fn from(other: Ipv4Addr) -> Self {
      Self::from_u32(other.to_bits())
    }
  }

  #[feature("const_convert")]
  impl const From<uint<16>> for Ipv6Addr {
    /// Uses [`Ipv6Addr::from_bits`] to convert a host byte order `uint<16>` to an IPv6 address.
    ///
    /// [`Ipv6Addr::from_bits`]: Ipv6Addr::from_bits
    #[inline]
    fn from(other: uint<16>) -> Self {
      Self::from_bits(other.into_u128())
    }
  }

  #[feature("const_convert")]
  impl const From<Ipv6Addr> for uint<16> {
    /// Uses [`Ipv6Addr::to_bits`] to convert an IPv6 address to a host byte order `uint<16>`.
    ///
    /// [`Ipv6Addr::to_bits`]: Ipv6Addr::to_bits
    #[inline]
    fn from(other: Ipv6Addr) -> Self {
      Self::from_u128(other.to_bits())
    }
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
    $doc:expr
  ) => {
    #[$meta]
    const _: () = {
      use ::core::sync::atomic::$atomic;

      const_trait_if! {
        #[feature("const_convert")]
        #[$meta]
        impl const From<$name<$size>> for $atomic {
          #[doc = $doc]
          #[inline]
          fn from(other: $name<$size>) -> Self {
            Self::new(other.$conv())
          }
        }
      }
    };
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
    $doc:expr
  ) => {
    const _: () = {
      use ::core::num::$nonzero;

      const_trait_if! {
        #[feature("const_convert")]
        impl const From<$nonzero> for $name<$size> {
          #[inline]
          fn from(other: $nonzero) -> Self {
            Self::$conv_from(other.get())
          }
        }

        #[feature("const_convert")]
        impl const TryFrom<$name<$size>> for $nonzero {
          type Error = TryFromIntError;

          #[doc = $doc]
          #[inline]
          fn try_from(other: $name<$size>) -> Result<Self, Self::Error> {
            $nonzero::new(other.$conv_into()).ok_or_else(TryFromIntError::new)
          }
        }
      }
    };
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
impl From<uint<1>> for ExitCode {
  /// Constructs an `ExitCode` from an arbitrary uint<1> value.
  #[inline]
  fn from(other: uint<1>) -> Self {
    Self::from(other.into_u8())
  }
}

// -----------------------------------------------------------------------------
// Crate Types
// -----------------------------------------------------------------------------

const_trait_if! {
  #[feature("const_convert")]
  impl<const N: usize, const M: usize> const TryFrom<int<M>> for uint<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(other: int<M>) -> Result<Self, Self::Error> {
      let min: int<M> = Cast::cast(Self::MIN);
      let max: int<M> = Cast::cast(Self::MAX);

      if other < min || other > max {
        Err(TryFromIntError::new())
      } else {
        Ok(Cast::cast(other))
      }
    }
  }

  #[feature("const_convert")]
  impl<const N: usize, const M: usize> const TryFrom<uint<M>> for int<N> {
    type Error = TryFromIntError;

    #[inline]
    fn try_from(other: uint<M>) -> Result<Self, Self::Error> {
      if other > Cast::cast(Self::MAX) {
        Err(TryFromIntError::new())
      } else {
        Ok(Cast::cast(other))
      }
    }
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
  (@from, $doc:expr, $from:ty, $into:ty) => {
    const_trait_if! {
      #[feature("const_convert")]
      impl const From<$from> for $into {
        #[doc = $doc]
        #[inline]
        fn from(other: $from) -> Self {
          Cast::cast(other)
        }
      }
    }
  };
}

macro_rules! impl_bounded {
  ($from:ty, $into:ty, |$binding:ident| $block:block) => {
    const_trait_if! {
      #[feature("const_convert")]
      impl const TryFrom<$from> for $into {
        type Error = TryFromIntError;

        /// Tries to create the target number type from a source number type.
        /// This returns an error if the source value is outside of the range of
        /// the target type.
        #[inline]
        fn try_from($binding: $from) -> Result<Self, Self::Error> {
          $block
        }
      }
    }
  };
  (@lower, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      if other >= Cast::cast(Self::MIN) {
        Ok(Cast::cast(other))
      } else {
        Err(TryFromIntError::new())
      }
    });
  };
  (@upper, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      if other > Cast::cast(Self::MAX) {
        Err(TryFromIntError::new())
      } else {
        Ok(Cast::cast(other))
      }
    });
  };
  (@range, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| {
      let min: $from = Cast::cast(Self::MIN);
      let max: $from = Cast::cast(Self::MAX);

      if other < min || other > max {
        Err(TryFromIntError::new())
      } else {
        Ok(Cast::cast(other))
      }
    });
  };
  (@none, $from:ty, $into:ty) => {
    impl_bounded!($from, $into, |other| { Ok(Cast::cast(other)) });
  };
}

macro_rules! impl_float {
  (
    $name:ident<$size:literal> as $integer:ty => $float:ty
    $(where $meta:meta)?
  ) => {
    const_trait_if! {
      #[feature("const_convert")]
      $(#[$meta])?
      impl const From<$name<$size>> for $float {
        #[allow(clippy::cast_precision_loss, reason = "false positive (upcast)")]
        #[doc = docstring!($name<$size>, $float)]
        #[inline]
        fn from(other: $name<$size>) -> Self {
          <$integer>::from(other) as $float
        }
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::primitive::*;
  use crate::tests::*;

  test!(@uint, test_ascii_char, () => {
    assert_eq!(T::P_0, T::from(Char::Null));
    assert_eq!(T::P_97, T::from(Char::SmallA));
    assert_eq!(T::P_127, T::from(Char::Delete));
  });

  test!(test_bool, () => {
    assert_eq!(T::P_1, T::from(true));
    assert_eq!(T::P_0, T::from(false));
  });

  test!(@uint(1), test_char, () => {
    assert_eq!(u8::P_97, u8::try_from('a').unwrap());
    assert_eq!(u16::P_97, u16::try_from('a').unwrap());
    assert_eq!(u24::P_97, u24::try_from('a').unwrap());

    assert_eq!(u32::P_97, u32::from('a'));
    assert_eq!(u40::P_97, u40::from('a'));
    assert_eq!(u48::P_97, u48::from('a'));
    assert_eq!(u56::P_97, u56::from('a'));
    assert_eq!(u64::P_97, u64::from('a'));
    assert_eq!(u72::P_97, u72::from('a'));
    assert_eq!(u80::P_97, u80::from('a'));
    assert_eq!(u88::P_97, u88::from('a'));
    assert_eq!(u96::P_97, u96::from('a'));
    assert_eq!(u104::P_97, u104::from('a'));
    assert_eq!(u112::P_97, u112::from('a'));
    assert_eq!(u120::P_97, u120::from('a'));
    assert_eq!(u128::P_97, u128::from('a'));

    assert_eq!('a', char::from(u8::P_97));
    assert_eq!('a', char::try_from(u32::P_97).unwrap());
  });

  test!(@uint(4), test_ipv4, () => {
    let address: Ipv4Addr = Ipv4Addr::new(0xAA, 0xBB, 0xCC, 0xDD);
    let integer: T = T::from_u32(0xAABBCCDD);

    assert_eq!(address, Ipv4Addr::from(integer));
    assert_eq!(integer, T::from(address));
  });

  test!(@uint(16), test_ipv6, () => {
    let address: Ipv6Addr = Ipv6Addr::new(0xAABB, 0xCCDD, 0xEEFF, 0xAABB, 0xCCDD, 0xEEFF, 0xAABB, 0xCCDD);
    let integer: T = T::from_u128(0xAABBCCDDEEFFAABBCCDDEEFFAABBCCDD);

    assert_eq!(address, Ipv6Addr::from(integer));
    assert_eq!(integer, T::from(address));
  });

  macro_rules! assert_atomic {
    ($name:ident, $atomic:ident) => {{
      use ::core::sync::atomic::$atomic;

      assert_eq!($atomic::from($name::P_0).into_inner(), 0);
      assert_eq!($atomic::from($name::P_1).into_inner(), 1);
      assert_eq!($atomic::from($name::P_2).into_inner(), 2);
      assert_eq!($atomic::from($name::P_3).into_inner(), 3);
    }};
  }

  test!(@sint(1), test_atomic_i8, () => {
    assert_atomic!(i8, AtomicI8);
  });

  test!(@sint(2), test_atomic_i16, () => {
    assert_atomic!(i16, AtomicI16);
  });

  test!(@sint(4), test_atomic_i32, () => {
    assert_atomic!(i32, AtomicI32);
  });

  test!(@sint(8), test_atomic_i64, () => {
    assert_atomic!(i64, AtomicI64);
  });

  #[cfg(all(target_has_atomic = "128", feature = "integer_atomics"))]
  test!(@sint(16), test_atomic_i128, () => {
    assert_atomic!(i128, AtomicI128);
  });

  test!(@uint(1), test_atomic_u8, () => {
    assert_atomic!(u8, AtomicU8);
  });

  test!(@uint(2), test_atomic_u16, () => {
    assert_atomic!(u16, AtomicU16);
  });

  test!(@uint(4), test_atomic_u32, () => {
    assert_atomic!(u32, AtomicU32);
  });

  test!(@uint(8), test_atomic_u64, () => {
    assert_atomic!(u64, AtomicU64);
  });

  #[cfg(all(target_has_atomic = "128", feature = "integer_atomics"))]
  test!(@uint(16), test_atomic_u128, () => {
    assert_atomic!(u128, AtomicU128);
  });

  macro_rules! assert_nonzero {
    ($name:ident, $nonzero:ident) => {{
      use ::core::num::$nonzero;

      const NZ_1: $nonzero = unsafe { $nonzero::new_unchecked(1) };
      const NZ_2: $nonzero = unsafe { $nonzero::new_unchecked(2) };
      const NZ_3: $nonzero = unsafe { $nonzero::new_unchecked(3) };

      assert_eq!($name::from(NZ_1), $name::P_1);
      assert_eq!($name::from(NZ_2), $name::P_2);
      assert_eq!($name::from(NZ_3), $name::P_3);

      assert_eq!($nonzero::try_from($name::P_0).is_err(), true);
      assert_eq!($nonzero::try_from($name::P_1).unwrap().get(), 1);
      assert_eq!($nonzero::try_from($name::P_2).unwrap().get(), 2);
      assert_eq!($nonzero::try_from($name::P_3).unwrap().get(), 3);
    }};
  }

  test!(@sint(1), test_nonzero_i8, () => {
    assert_nonzero!(i8, NonZeroI8)
  });

  test!(@sint(2), test_nonzero_i16, () => {
    assert_nonzero!(i16, NonZeroI16)
  });

  test!(@sint(4), test_nonzero_i32, () => {
    assert_nonzero!(i32, NonZeroI32)
  });

  test!(@sint(8), test_nonzero_i64, () => {
    assert_nonzero!(i64, NonZeroI64)
  });

  test!(@sint(16), test_nonzero_i128, () => {
    assert_nonzero!(i128, NonZeroI128)
  });

  test!(@uint(1), test_nonzero_u8, () => {
    assert_nonzero!(u8, NonZeroU8)
  });

  test!(@uint(2), test_nonzero_u16, () => {
    assert_nonzero!(u16, NonZeroU16)
  });

  test!(@uint(4), test_nonzero_u32, () => {
    assert_nonzero!(u32, NonZeroU32)
  });

  test!(@uint(8), test_nonzero_u64, () => {
    assert_nonzero!(u64, NonZeroU64)
  });

  test!(@uint(16), test_nonzero_u128, () => {
    assert_nonzero!(u128, NonZeroU128)
  });
}
