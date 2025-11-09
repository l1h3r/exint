//! Test Utilities

#![allow(dead_code, reason = "tests")]
#![allow(unused, reason = "tests")]

mod assert_ilog;
mod assert_isqrt;
mod assert_panic;
mod assert_shift;
mod integer;

pub(crate) use ::std::option::Option;
pub(crate) use ::std::option::Option::None;
pub(crate) use ::std::option::Option::Some;

pub(crate) use ::std::result::Result;
pub(crate) use ::std::result::Result::Err;
pub(crate) use ::std::result::Result::Ok;

pub(crate) use ::std::assert_eq;
pub(crate) use ::std::assert_ne;
pub(crate) use ::std::format;

pub(crate) use self::assert_ilog::assert_ilog;
pub(crate) use self::assert_isqrt::assert_isqrt;
pub(crate) use self::assert_panic::assert_panic;
pub(crate) use self::assert_shift::assert_checked_shift;
pub(crate) use self::assert_shift::assert_overflowing_shift;
pub(crate) use self::assert_shift::assert_shift;
pub(crate) use self::assert_shift::assert_strict_shift;
pub(crate) use self::assert_shift::assert_unbounded_shift;
pub(crate) use self::assert_shift::assert_wrapping_shift;
pub(crate) use self::integer::IntExt;

// -----------------------------------------------------------------------------
// test!
// -----------------------------------------------------------------------------

macro_rules! run_one {
  ($integer:ident, $inverse:ident, $block:block, $size:literal) => {{
    #![allow(clippy::bool_assert_comparison)]

    #[allow(dead_code, reason = "macro-generated")]
    type T = $crate::types::$integer<$size>;

    #[allow(dead_code, reason = "macro-generated")]
    type U = $crate::types::$inverse<$size>;

    $block
  }};
}

#[cfg(all(miri, not(feature = "__internal_test_all_custom_sizes")))]
macro_rules! run_all {
  ($integer:ident, $inverse:ident, $block:block) => {
    $crate::tests::run_one!($integer, $inverse, $block, 3);
    $crate::tests::run_one!($integer, $inverse, $block, 4);
  };
}

#[cfg(not(all(miri, feature = "__internal_test_all_custom_sizes")))]
macro_rules! run_all {
  ($integer:ident, $inverse:ident, $block:block) => {
    $crate::tests::run_one!($integer, $inverse, $block, 3);
    $crate::tests::run_one!($integer, $inverse, $block, 4);
    $crate::tests::run_one!($integer, $inverse, $block, 31);
  };
}

#[cfg(feature = "__internal_test_all_custom_sizes")]
macro_rules! run_all {
  ($integer:ident, $inverse:ident, $block:block) => {
    $crate::tests::run_one!($integer, $inverse, $block, 1);
    $crate::tests::run_one!($integer, $inverse, $block, 2);
    $crate::tests::run_one!($integer, $inverse, $block, 4);
    $crate::tests::run_one!($integer, $inverse, $block, 8);
    $crate::tests::run_one!($integer, $inverse, $block, 16);

    $crate::tests::run_one!($integer, $inverse, $block, 3);
    $crate::tests::run_one!($integer, $inverse, $block, 5);
    $crate::tests::run_one!($integer, $inverse, $block, 6);
    $crate::tests::run_one!($integer, $inverse, $block, 7);
    $crate::tests::run_one!($integer, $inverse, $block, 9);
    $crate::tests::run_one!($integer, $inverse, $block, 10);
    $crate::tests::run_one!($integer, $inverse, $block, 11);
    $crate::tests::run_one!($integer, $inverse, $block, 12);
    $crate::tests::run_one!($integer, $inverse, $block, 13);
    $crate::tests::run_one!($integer, $inverse, $block, 14);
    $crate::tests::run_one!($integer, $inverse, $block, 15);

    $crate::tests::run_one!($integer, $inverse, $block, 31);
    $crate::tests::run_one!($integer, $inverse, $block, 32);
    $crate::tests::run_one!($integer, $inverse, $block, 48);
    $crate::tests::run_one!($integer, $inverse, $block, 64);
    $crate::tests::run_one!($integer, $inverse, $block, 128);
  };
}

macro_rules! run_set {
  ($block:block) => {
    $crate::tests::run_all!(int, uint, $block);
    $crate::tests::run_all!(uint, int, $block);
  };
  (@sint $block:block) => {
    $crate::tests::run_all!(int, uint, $block);
  };
  (@uint $block:block) => {
    $crate::tests::run_all!(uint, int, $block);
  };
  (@sint($size:literal) $block:block) => {
    $crate::tests::run_one!(int, uint, $block, $size);
  };
  (@uint($size:literal) $block:block) => {
    $crate::tests::run_one!(uint, int, $block, $size);
  };
  (@size($size:literal) $block:block) => {
    $crate::tests::run_one!(uint, int, $block, $size);
    $crate::tests::run_one!(int, uint, $block, $size);
  };
}

macro_rules! test {
  ($(#[no_miri] $($miri:lifetime)?)? $(@$scope:ident $(($param:literal))?,)? $name:ident, () => $block:block) => {
    $(#[cfg_attr(miri, ignore)] $($miri)?)?
    #[test]
    fn $name() {
      $crate::tests::run_set!($(@$scope $(($param))?)? $block);
    }
  };
}

pub(crate) use run_all;
pub(crate) use run_one;
pub(crate) use run_set;
pub(crate) use test;
