macro_rules! implement {
  (
    $(#[$meta:meta])*
    pub struct $name:ident
    $(
      impl $_name:ident {
        $($tt:tt)*
      }
    )?
  ) => {
    $(#[$meta])*
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name<T>(pub T);

    // -------------------------------------------------------------------------
    // Implementation - uint
    // -------------------------------------------------------------------------

    impl<const N: usize> $name<$crate::uint<N>> {
      $crate::types::macros::internals!($name<uint>);
    }

    // Internal helper for types::traits::ops
    impl<const N: usize> ::core::convert::From<$crate::uint<N>> for $name<$crate::uint<N>> {
      #[doc(hidden)]
      #[inline]
      fn from(other: $crate::uint<N>) -> Self {
        Self(other)
      }
    }

    impl<const N: usize> $name<$crate::uint<N>> {
      #[doc = $crate::utils::include_doc!($name, uint, "BITS")]
      pub const BITS: u32 = $crate::uint::<N>::BITS;

      #[doc = $crate::utils::include_doc!($name, uint, "MAX")]
      pub const MAX: Self = Self($crate::uint::<N>::MAX);

      #[doc = $crate::utils::include_doc!($name, uint, "MIN")]
      pub const MIN: Self = Self($crate::uint::<N>::MIN);
    }

    impl<const N: usize> $name<$crate::uint<N>> {
      $crate::types::macros::byteorder!($name<uint>);
    }

    impl<const N: usize> $name<$crate::uint<N>> {
      $crate::types::macros::bin_tools!($name<uint>);
    }

    $(
      impl<const N: usize> $name<$crate::uint<N>> {
        $($tt)*
      }
    )?

    // -------------------------------------------------------------------------
    // Implementation - int
    // -------------------------------------------------------------------------

    impl<const N: usize> $name<$crate::int<N>> {
      $crate::types::macros::internals!($name<int>);
    }

    // Internal helper for types::traits::ops
    impl<const N: usize> ::core::convert::From<$crate::int<N>> for $name<$crate::int<N>> {
      #[doc(hidden)]
      #[inline]
      fn from(other: $crate::int<N>) -> Self {
        Self(other)
      }
    }

    impl<const N: usize> $name<$crate::int<N>> {
      #[doc = $crate::utils::include_doc!($name, int, "BITS")]
      pub const BITS: u32 = $crate::int::<N>::BITS;

      #[doc = $crate::utils::include_doc!($name, int, "MAX")]
      pub const MAX: Self = Self($crate::int::<N>::MAX);

      #[doc = $crate::utils::include_doc!($name, int, "MIN")]
      pub const MIN: Self = Self($crate::int::<N>::MIN);
    }

    impl<const N: usize> $name<$crate::int<N>> {
      $crate::types::macros::byteorder!($name<int>);
    }

    impl<const N: usize> $name<$crate::int<N>> {
      $crate::types::macros::bin_tools!($name<int>);
    }

    $(
      impl<const N: usize> $name<$crate::int<N>> {
        $($tt)*
      }
    )?
  };
}

implement! {
  /// Provides intentionally-saturating arithmetic on `T`.
  ///
  /// Operations like `+` on `u32` values are intended to never overflow, and
  /// in some debug configurations overflow is detected and results in a panic.
  /// While most arithmetic falls into this category, some code explicitly
  /// expects and relies upon saturating arithmetic.
  ///
  /// Saturating arithmetic can be achieved either through methods like
  /// `saturating_add`, or through the `Saturating<T>` type, which says that all
  /// standard arithmetic operations on the underlying value are intended to
  /// have saturating semantics.
  ///
  /// The underlying value can be retrieved through the `.0` index of the
  /// `Saturating` tuple.
  ///
  /// # Layout
  ///
  /// `Saturating<T>` is guaranteed to have the same layout and ABI as `T`.
  pub struct Saturating

  impl Saturating {
    #[must_use = crate::utils::must_use_doc!()]
    #[inline]
    pub const fn pow(self, exp: u32) -> Self {
      Self(self.0.saturating_pow(exp))
    }
  }
}

implement! {
  /// Provides intentionally-strict arithmetic on `T`.
  ///
  /// Operations like `+` on `u32` values are intended to never overflow, and
  /// in some debug configurations overflow is detected and results in a panic.
  /// While most arithmetic falls into this category, some code explicitly
  /// expects and relies upon strict arithmetic.
  ///
  /// Strict arithmetic can be achieved either through methods like
  /// `strict_add`, or through the `Strict<T>` type, which says that all
  /// standard arithmetic operations on the underlying value are intended to
  /// have strict semantics.
  ///
  /// The underlying value can be retrieved through the `.0` index of the
  /// `Strict` tuple.
  ///
  /// # Layout
  ///
  /// `Strict<T>` is guaranteed to have the same layout and ABI as `T`.
  pub struct Strict

  impl Strict {
    #[must_use = crate::utils::must_use_doc!()]
    #[inline]
    pub const fn pow(self, exp: u32) -> Self {
      Self(self.0.strict_pow(exp))
    }
  }
}

implement! {
  /// Provides intentionally-wrapped arithmetic on `T`.
  ///
  /// Operations like `+` on `u32` values are intended to never overflow, and
  /// in some debug configurations overflow is detected and results in a panic.
  /// While most arithmetic falls into this category, some code explicitly
  /// expects and relies upon modular arithmetic (e.g., hashing).
  ///
  /// Wrapping arithmetic can be achieved either through methods like
  /// `wrapping_add`, or through the `Wrapping<T>` type, which says that all
  /// standard arithmetic operations on the underlying value are intended to
  /// have wrapping semantics.
  ///
  /// The underlying value can be retrieved through the `.0` index of the
  /// `Wrapping` tuple.
  ///
  /// # Layout
  ///
  /// `Wrapping<T>` is guaranteed to have the same layout and ABI as `T`.
  pub struct Wrapping

  impl Wrapping {
    #[must_use = crate::utils::must_use_doc!()]
    #[inline]
    pub const fn pow(self, exp: u32) -> Self {
      Self(self.0.wrapping_pow(exp))
    }
  }
}
