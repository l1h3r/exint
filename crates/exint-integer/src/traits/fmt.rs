macro_rules! fmt {
  ($name:ident) => {
    $crate::traits::fmt!(impl Binary   for $name);
    $crate::traits::fmt!(impl Debug    for $name);
    $crate::traits::fmt!(impl Display  for $name);
    $crate::traits::fmt!(impl LowerExp for $name);
    $crate::traits::fmt!(impl LowerHex for $name);
    $crate::traits::fmt!(impl Octal    for $name);
    $crate::traits::fmt!(impl UpperExp for $name);
    $crate::traits::fmt!(impl UpperHex for $name);
  };
  (impl $format:ident for $name:ident) => {
    impl<const S: usize> ::core::fmt::$format for $name<S> {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        panic!(concat!(stringify!($fmt), "::fmt"))
      }
    }
  };
}

pub(crate) use fmt;
