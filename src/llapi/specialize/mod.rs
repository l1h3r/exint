//! Specialized Implementations

mod extended;
mod fallback;
mod standard;

pub(crate) use self::extended::I24;
pub(crate) use self::extended::I40;
pub(crate) use self::extended::I48;
pub(crate) use self::extended::I56;
pub(crate) use self::extended::I72;
pub(crate) use self::extended::I80;
pub(crate) use self::extended::I88;
pub(crate) use self::extended::I96;
pub(crate) use self::extended::I104;
pub(crate) use self::extended::I112;
pub(crate) use self::extended::I120;
pub(crate) use self::fallback::Int;
pub(crate) use self::standard::I8;
pub(crate) use self::standard::I16;
pub(crate) use self::standard::I32;
pub(crate) use self::standard::I64;
pub(crate) use self::standard::I128;

#[cfg(feature = "portable_simd")]
mod parallel;

#[cfg(feature = "portable_simd")]
pub(crate) use self::parallel::I64xN;
