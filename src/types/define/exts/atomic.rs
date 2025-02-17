mod private {
  #[allow(unnameable_types)]
  pub trait Sealed {}
}

/// Convert a generic integer to an equivalent atomic integer type.
pub trait ToAtomic: private::Sealed {
  /// The equivalent atomic integer type.
  type Atomic;

  /// Creates a new atomic integer from `self`.
  fn to_atomic(self) -> Self::Atomic;
}

macro_rules! implement {
  (
    $name:ident<$size:literal>,
    $conv:ident,
    $atomic:ident,
    $meta:meta
  ) => {
    impl $crate::$name<$size> {
      /// Creates a new atomic integer from `self`.
      #[$meta]
      #[must_use]
      #[inline]
      pub const fn to_atomic(self) -> ::core::sync::atomic::$atomic {
        ::core::sync::atomic::$atomic::new(self.$conv())
      }
    }

    #[$meta]
    impl private::Sealed for $crate::$name<$size> {}

    #[$meta]
    impl ToAtomic for $crate::$name<$size> {
      type Atomic = ::core::sync::atomic::$atomic;

      #[must_use]
      #[inline]
      fn to_atomic(self) -> Self::Atomic {
        self.to_atomic()
      }
    }
  };
}

implement! {
  uint<1>,
  into_u8,
  AtomicU8,
  cfg(target_has_atomic = "8")
}

implement! {
  uint<2>,
  into_u16,
  AtomicU16,
  cfg(target_has_atomic = "16")
}

implement! {
  uint<4>,
  into_u32,
  AtomicU32,
  cfg(target_has_atomic = "32")
}

implement! {
  uint<8>,
  into_u64,
  AtomicU64,
  cfg(target_has_atomic = "64")
}

implement! {
  uint<16>,
  into_u128,
  AtomicU128,
  cfg(all(target_has_atomic = "128", feature = "integer_atomics"))
}

implement! {
  int<1>,
  into_i8,
  AtomicI8,
  cfg(target_has_atomic = "8")
}

implement! {
  int<2>,
  into_i16,
  AtomicI16,
  cfg(target_has_atomic = "16")
}

implement! {
  int<4>,
  into_i32,
  AtomicI32,
  cfg(target_has_atomic = "32")
}

implement! {
  int<8>,
  into_i64,
  AtomicI64,
  cfg(target_has_atomic = "64")
}

implement! {
  int<16>,
  into_i128,
  AtomicI128,
  cfg(all(target_has_atomic = "128", feature = "integer_atomics"))
}
