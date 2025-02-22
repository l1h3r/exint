macro_rules! implement {
  (impl $format:ident for $name:ident/$conv:ident) => {
    impl<const N: usize> ::core::fmt::$format for $crate::$name<N> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if Self::BITS > u128::BITS {
          ::core::panic!("TODO - Format Bignum")
        } else {
          ::core::fmt::$format::fmt(&self.$conv(), f)
        }
      }
    }
  };
  (impl $format:ident for $outer:ident<T>) => {
    impl<T: ::core::fmt::$format> ::core::fmt::$format for $crate::$outer<T> {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <T as ::core::fmt::$format>::fmt(&self.0, f)
      }
    }
  };
  (int) => {
    implement!(impl Binary   for int/into_i128);
    implement!(impl Debug    for int/into_i128);
    implement!(impl Display  for int/into_i128);
    implement!(impl LowerExp for int/into_i128);
    implement!(impl LowerHex for int/into_i128);
    implement!(impl Octal    for int/into_i128);
    implement!(impl UpperExp for int/into_i128);
    implement!(impl UpperHex for int/into_i128);
  };
  (uint) => {
    implement!(impl Binary   for uint/into_u128);
    implement!(impl Debug    for uint/into_u128);
    implement!(impl Display  for uint/into_u128);
    implement!(impl LowerExp for uint/into_u128);
    implement!(impl LowerHex for uint/into_u128);
    implement!(impl Octal    for uint/into_u128);
    implement!(impl UpperExp for uint/into_u128);
    implement!(impl UpperHex for uint/into_u128);
  };
  ($outer:ident<T>) => {
    implement!(impl Binary   for $outer<T>);
    implement!(impl Debug    for $outer<T>);
    implement!(impl Display  for $outer<T>);
    implement!(impl LowerExp for $outer<T>);
    implement!(impl LowerHex for $outer<T>);
    implement!(impl Octal    for $outer<T>);
    implement!(impl UpperExp for $outer<T>);
    implement!(impl UpperHex for $outer<T>);
  };
}

implement!(int);
implement!(uint);

implement!(Saturating<T>);
#[cfg(feature = "strict_overflow_ops")]
implement!(Strict<T>);
implement!(Wrapping<T>);
