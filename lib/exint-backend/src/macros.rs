/// Cast between a generic integer type and a byte array of known size.
///
/// See [`transmute`] for more information.
///
/// [`transmute`]: crate::utils::transmute
macro_rules! cast {
  (N $expr:expr_2021) => {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<T, [u8; N]>($expr) }
  };
  (T $expr:expr_2021) => {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { $crate::utils::transmute::<[u8; N], T>($expr) }
  };
  ((T, bool) $expr:expr_2021) => {{
    let out: ([u8; N], bool) = $expr;
    let int: T = $crate::macros::cast!(T out.0);

    (int, out.1)
  }};
}

/// A wrapper around `impl const $trait` to avoid emitting unused tokens that
/// don't compile on stable rust.
#[cfg(feature = "const_trait_impl")]
macro_rules! const_trait_impl {
  ($($tt:tt)*) => {
    $($tt)*
  };
}

/// A wrapper around `impl const $trait` to avoid emitting unused tokens that
/// don't compile on stable rust.
#[cfg(not(feature = "const_trait_impl"))]
macro_rules! const_trait_impl {
  ($($tt:tt)*) => {};
}

/// Use the `enabled` branch when `core_intrinsics` are available,
/// otherwise use the `default` fallback implementation.
macro_rules! maybe_intrinsic {
  (
    @enabled => $($enabled_safety:ident)? {
      $($enabled:tt)+
    }
    @default => $($default_safety:ident)? {
      $($default:tt)+
    }
  ) => {
    #[cfg(feature = "core_intrinsics")]
    $($enabled_safety)? { $($enabled)+}
    #[cfg(not(feature = "core_intrinsics"))]
    $($default_safety)? { $($default)+ }
  };
}

/// Implement a specialized `const` trait for multiple explicit integer sizes
/// with identical implementations.
#[cfg(feature = "min_specialization")]
macro_rules! specialize {
  (impl const $trait:ident for Int<$size:literal> { $($tt:tt)+ }) => {
    $crate::macros::const_trait_impl! {
      impl const $trait for [u8; $size] {
        $($tt)+
      }
    }
  };
  (impl const $trait:ident for Int<$head:literal $(| $tail:literal)*> { $($tt:tt)+ }) => {
    $crate::macros::specialize!(impl const $trait for Int<$head> { $($tt)+ });
    $crate::macros::specialize!(impl const $trait for Int<$($tail)|*> { $($tt)+ });
  };
  (impl const $trait:ident for Int<$type:ty> { $($tt:tt)+ }) => {
    $crate::macros::const_trait_impl! {
      impl const $trait for $type {
        $($tt)+
      }
    }
  };
  (impl const $trait:ident for Int<$head:ty $(| $tail:ty)*> { $($tt:tt)+ }) => {
    $crate::macros::specialize!(impl const $trait for Int<$head> { $($tt)+ });
    $crate::macros::specialize!(impl const $trait for Int<$($tail)|*> { $($tt)+ });
  };
}

pub(crate) use cast;
pub(crate) use const_trait_impl;
pub(crate) use maybe_intrinsic;

#[cfg(feature = "min_specialization")]
pub(crate) use specialize;
