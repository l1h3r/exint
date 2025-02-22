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

pub(crate) use arithmetic_select;
