//! Exint Library Tests

#![feature(panic_internals)]
#![allow(internal_features)]

#[macro_use]
extern crate exint;

use exint::IntErrorKind;

pub(crate) const BAD_STRINGS: &'static [(&'static str, IntErrorKind)] = &[
  ("+", IntErrorKind::InvalidDigit),
  ("-", IntErrorKind::InvalidDigit),

  ("0 ", IntErrorKind::InvalidDigit),
  (" 0", IntErrorKind::InvalidDigit),

  ("0z", IntErrorKind::InvalidDigit),
  ("z0", IntErrorKind::InvalidDigit),

  ("0_", IntErrorKind::InvalidDigit),
  ("_0", IntErrorKind::InvalidDigit)
  ,
  ("", IntErrorKind::Empty),
  (" ", IntErrorKind::InvalidDigit),
  ("0_0", IntErrorKind::InvalidDigit),
  ("owo", IntErrorKind::InvalidDigit),
];

// TODO: Replace with `core::assert_matches` once stable
macro_rules! assert_matches {
  ($lhs:expr, $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => {
    match $lhs {
      $($pattern)|+ $(if $guard)? => {}
      ref lhs => {
        ::core::panicking::assert_matches_failed(
          lhs,
          ::core::stringify!($($pattern)|+ $(if $guard)?),
          ::core::option::Option::None
        );

        // assert!(matches!($lhs, $($pattern)|+ $(if $guard)?));
      }
    }
  };
}

macro_rules! assert_impls {
  ($type:ty: $trait:ident $(,)?) => {{
    #![allow(dead_code)]

    const _: () = {
      trait Fallback {
        const YES: bool = false;
      }

      impl<T> Fallback for T {}

      struct HasTrait<T>(core::marker::PhantomData<T>);

      impl<T: $trait> HasTrait<T> {
        const YES: bool = true;
      }

      assert!(HasTrait::<$type>::YES);
    };
  }};
}

mod bin_tools;
mod byteorder;
mod constants;
mod core_traits;
mod parse_int;
mod parse_str;
