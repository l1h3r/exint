// An attempt to match the default semantics of overflow behaviour.
//
// Ref: https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow
//
// TODO: Try using #[cfg(overflow_checks)]: https://github.com/rust-lang/rust/issues/111466
macro_rules! arithmetic_select {
  (
    message: $message:expr,
    checked: $checked:expr,
    wrapped: $wrapped:expr,
  ) => {
    #[cfg(debug_assertions)]
    { $checked.expect($message) }

    #[cfg(not(debug_assertions))]
    { $wrapped }
  };
}

macro_rules! cast {
  ($from:ty as $into:ty, $expr:expr) => {
    // Note: This fails with "generic `Self` types are currently not permitted in anonymous constants"
    // <$into>::from_ne_bytes($crate::intrinsics::cast::<
    //   { <$from as $crate::value::Value>::SIZE },
    //   { <$into as $crate::value::Value>::SIZE },
    //   { <$from as $crate::value::Value>::UINT },
    // >($expr.to_ne_bytes()))

    // TODO: Check if this branch is actually optimized out
    if <$from as $crate::value::Value>::UINT {
      <$into>::from_ne_bytes($crate::intrinsics::cast::<_, _, true>($expr.to_ne_bytes()))
    } else {
      <$into>::from_ne_bytes($crate::intrinsics::cast::<_, _, false>($expr.to_ne_bytes()))
    }
  };
}

pub(crate) use arithmetic_select;
pub(crate) use cast;
