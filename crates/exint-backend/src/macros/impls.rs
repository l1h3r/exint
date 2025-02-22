/// Implement a specialized const trait for multiple integer sizes.
macro_rules! specialize {
  (impl $trait:ident for Int<$size:literal> { $($tt:tt)+ }) => {
    impl const $trait for $crate::types::Int<$size> {
      $($tt)+
    }
  };
  (impl $trait:ident for Int<$head:literal $(| $tail:literal)*> { $($tt:tt)+ }) => {
    $crate::macros::specialize!(impl $trait for Int<$head> { $($tt)+ });
    $crate::macros::specialize!(impl $trait for Int<$($tail)|*> { $($tt)+ });
  };
  (impl $trait:ident for Int<> { $($tt:tt)+ }) => {};
}

pub(crate) use specialize;
