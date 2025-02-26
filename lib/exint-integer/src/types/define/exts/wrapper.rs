macro_rules! implement {
  (
    $(#[$meta:meta])*
    pub struct Strict;
    forward {
      $(#[$pow_docs:meta])+
      pow = $pow:ident;
      $(#[$abs_docs:meta])+
      abs = $abs:ident;
    }
  ) => {
    #[cfg(feature = "strict_overflow_ops")]
    $(#[$meta])*
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct Strict<T>(pub T);

    #[cfg(feature = "strict_overflow_ops")]
    implement! {
      @implement
      pub struct Strict;
      forward {
        $(#[$pow_docs])+
        pow = $pow;
        $(#[$abs_docs])+
        abs = $abs;
      }
    }
  };
  (
    $(#[$meta:meta])*
    pub struct $name:ident;
    forward {
      $(#[$pow_docs:meta])+
      pow = $pow:ident;
      $(#[$abs_docs:meta])+
      abs = $abs:ident;
    }
  ) => {
    $(#[$meta])*
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name<T>(pub T);

    implement! {
      @implement
      pub struct $name;
      forward {
        $(#[$pow_docs])+
        pow = $pow;
        $(#[$abs_docs])+
        abs = $abs;
      }
    }
  };
  (
    @implement
    $(#[$meta:meta])*
    pub struct $name:ident;
    forward {
      $(#[$pow_docs:meta])+
      pow = $pow:ident;
      $(#[$abs_docs:meta])+
      abs = $abs:ident;
    }
  ) => {
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

    impl<const N: usize> $name<$crate::uint<N>> {
      $(#[$pow_docs])+
      #[must_use = $crate::utils::must_use_doc!()]
      #[inline]
      pub const fn pow(self, exp: u32) -> Self {
        Self(self.0.$pow(exp))
      }
    }

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

    impl<const N: usize> $name<$crate::int<N>> {
      $(#[$pow_docs])+
      #[must_use = $crate::utils::must_use_doc!()]
      #[inline]
      pub const fn pow(self, exp: u32) -> Self {
        Self(self.0.$pow(exp))
      }

      $(#[$abs_docs])+
      #[must_use = $crate::utils::must_use_doc!()]
      #[inline]
      pub const fn abs(self) -> Self {
        Self(self.0.$abs())
      }
    }
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
  pub struct Saturating;

  forward {
    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, Saturating};
    ///
    /// assert_eq!(Saturating(int!(3)).pow(4), Saturating(int!(81)));
    /// ```
    ///
    /// Results that are too large are saturated:
    ///
    /// ```
    /// use exint::{int, Saturating};
    ///
    /// assert_eq!(Saturating(int!(3 i8)).pow(5), Saturating(int!(127 i8)));
    /// assert_eq!(Saturating(int!(3 i8)).pow(6), Saturating(int!(127 i8)));
    /// ```
    pow = saturating_pow;
    /// Saturating absolute value. Computes `self.0.abs()`,
    /// returning `MAX` if `self == MIN` instead of overflowing.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, Saturating};
    ///
    /// assert_eq!(Saturating(int!(100)).abs(), Saturating(int!(100)));
    /// assert_eq!(Saturating(int!(-100)).abs(), Saturating(int!(100)));
    /// assert_eq!(Saturating(<int>::MIN).abs(), Saturating((<int>::MIN + int!(1)).abs()));
    /// assert_eq!(Saturating(<int>::MIN).abs(), Saturating(<int>::MIN.saturating_abs()));
    /// assert_eq!(Saturating(<int>::MIN).abs(), Saturating(<int>::MAX));
    /// ```
    abs = saturating_abs;
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
  pub struct Strict;

  forward {
    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, Strict};
    ///
    /// assert_eq!(Strict(int!(3)).pow(4), Strict(int!(81)));
    /// ```
    ///
    /// Results that are too large trigger a panic:
    ///
    /// ```should_panic
    /// use exint::{int, Strict};
    ///
    /// let _ = Strict(int!(3 i8)).pow(5);
    /// ```
    pow = strict_pow;
    /// Strict absolute value. Computes `self.0.abs()`,
    /// panicking if `self == MIN`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, Strict};
    ///
    /// assert_eq!(Strict(int!(-5)).abs(), Strict(int!(5)));
    /// ```
    ///
    /// The following panics because of overflow:
    ///
    /// ```should_panic
    /// use exint::{int, Strict};
    ///
    /// let _ = Strict(<int>::MIN).abs();
    /// ```
    abs = strict_abs;
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
  pub struct Wrapping;

  forward {
    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, Wrapping};
    ///
    /// assert_eq!(Wrapping(int!(3)).pow(4), Wrapping(int!(81)));
    /// ```
    ///
    /// Results that are too large are wrapped:
    ///
    /// ```
    /// use exint::{int, Wrapping};
    ///
    /// assert_eq!(Wrapping(int!(3 i8)).pow(5), Wrapping(int!(-13 i8)));
    /// assert_eq!(Wrapping(int!(3 i8)).pow(6), Wrapping(int!(-39 i8)));
    /// ```
    pow = wrapping_pow;
    /// Wrapping (modular) absolute value. Computes `self.0.abs()`,
    /// wrapping around at the boundary of the type.
    ///
    /// The only case where such wrapping can occur is when one takes the
    /// absolute value of the negative minimal value for the type; this is a
    /// positive value that is too large to represent in the type. In such a
    /// case, this function returns `MIN` itself.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use exint::{int, uint, Wrapping};
    ///
    /// assert_eq!(Wrapping(int!(100)).abs(), Wrapping(int!(100)));
    /// assert_eq!(Wrapping(int!(-100)).abs(), Wrapping(int!(100)));
    /// assert_eq!(Wrapping(<int>::MIN).abs(), Wrapping(<int>::MIN));
    /// ```
    // TODO: FIXME
    // assert_eq!(Wrapping(int!(-128)).abs().cast_unsigned(), Wrapping(uint!(128)));
    abs = wrapping_abs;
  }
}
