/// An attempt to match the default semantics of overflow behavior.
///
/// [Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow)
macro_rules! arithmetic_select {
  (
    message: $message:expr_2021,
    checked: $checked:expr_2021,
    wrapped: $wrapped:expr_2021,
  ) => {
    // TODO: Try #[cfg(overflow_checks)]: https://github.com/rust-lang/rust/issues/111466
    #[cfg(debug_assertions)]
    match $checked {
      Some(result) => result,
      None => $message(),
    }

    #[cfg(not(debug_assertions))]
    $wrapped
  };
}

/// Alter the visibility of an unstable function used internally.
macro_rules! stability {
  (
    #[unstable(feature = $feature:literal)]
    $(#[$meta:meta])*
    pub const $(unsafe $($unsafe:lifetime)?)? fn $name:ident $($tt:tt)+
  ) => {
    #[cfg(feature = $feature)]
    $(#[$meta])*
    pub const $(unsafe $($unsafe)?)? fn $name $($tt)+

    #[cfg(not(feature = $feature))]
    $(#[$meta])*
    pub(crate) const $(unsafe $($unsafe)?)? fn $name $($tt)+
  };
}

/// Option-specific version of `core::ops::Try` that works in const functions.
macro_rules! tri {
  ($expr:expr_2021) => {
    $crate::types::macros::tri!($expr, return None)
  };
  ($expr:expr_2021, $none:expr_2021) => {
    match $expr {
      Some(value) => value,
      None => $none,
    }
  };
}

pub(crate) use arithmetic_select;
pub(crate) use stability;
pub(crate) use tri;
