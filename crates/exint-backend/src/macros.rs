//! Common macros, for internal use only.

/// Cast between a generic integer type and a byte array of known size.
///
/// See [`transmute`] for more information.
///
/// [`transmute`]: crate::utils::transmute
macro_rules! cast {
  (S, $expr:expr) => {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<T, $crate::types::Integer<S>>($expr) }
  };
  (T, $expr:expr) => {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<$crate::types::Integer<S>, T>($expr) }
  };
  ((T, bool), $expr:expr) => {{
    let out: ($crate::types::Integer<S>, bool) = $expr;
    let num: T = $crate::macros::cast!(T, out.0);

    (num, out.1)
  }};
}

/// Provides a `for`-loop style syntax that can be used in `const` functions.
macro_rules! cfor {
  (@for $index:ident in $lower:tt..$upper:tt $block:block) => {{
    let mut $index: usize = $lower;

    while $index < $upper {
      $block
      $index += 1;
    }
  }};
}

/// A wrapper around [`cfor`] that runs an expression for each element in a byte
/// array, returning the results as new byte array.
macro_rules! cmap {
  (@for $index:ident in $lower:tt..$upper:tt do $expr:expr) => {{
    let mut data: $crate::types::Integer<$upper> = $crate::traits::Consts::UMIN;

    $crate::macros::cfor! {
      @for $index in $lower..$upper {
        data[$index] = $expr;
      }
    }

    data
  }};
}

/// A wrapper around [`cfor`] that runs an expression for each element in a byte
/// array, returning the results in reverse as new byte array.
macro_rules! crev {
  (@for $index:ident in $lower:tt..$upper:tt do $expr:expr) => {{
    let mut data: $crate::types::Integer<$upper> = $crate::traits::Consts::UMIN;

    $crate::macros::cfor! {
      @for $index in $lower..$upper {
        data[$upper - $index - 1] = $expr;
      }
    }

    data
  }};
}

/// Implement a specialized `const` trait for multiple explicit integer sizes
/// with identical implementations.
macro_rules! specialize {
  (impl const $trait:ident for Integer<$size:literal> { $($tt:tt)* }) => {
    const _: () = {
      // Inject integer size as `S` for use with generic code.
      const S: usize = $size;

      impl const $crate::bridge::traits::$trait for $crate::types::Integer<$size> {
        $($tt)*
      }
    };
  };
  (impl const $trait:ident for Integer<$head:literal $(| $tail:literal)*> { $($tt:tt)* }) => {
    $crate::macros::specialize!(impl const $trait for Integer<$head> { $($tt)* });
    $crate::macros::specialize!(impl const $trait for Integer<$($tail)|*> { $($tt)* });
  };
  (impl const $trait:ident for Integer<> { $($tt:tt)* }) => {};
}

pub(crate) use cast;
pub(crate) use cfor;
pub(crate) use cmap;
pub(crate) use crev;
pub(crate) use specialize;
