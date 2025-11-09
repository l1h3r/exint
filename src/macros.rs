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
  ($expr:expr) => {
    tri!($expr, return ::core::option::Option::None)
  };
  ($expr:expr, $none:expr) => {
    match $expr {
      ::core::option::Option::Some(value) => value,
      ::core::option::Option::None => $none,
    }
  };
}

/// An attempt to match the default semantics of overflow behavior.
///
/// [Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow)
macro_rules! arithmetic_select {
  (
    message: $message:expr,
    checked: $checked:expr,
    wrapped: $wrapped:expr,
  ) => {
    #[cfg(any(debug_assertions, feature = "__internal_force_overflow_checks"))]
    match $checked {
      ::core::option::Option::Some(result) => result,
      ::core::option::Option::None => $message(),
    }

    #[cfg(not(any(debug_assertions, feature = "__internal_force_overflow_checks")))]
    $wrapped
  };
}

macro_rules! include_doc {
  ($type:ident, $name:literal) => {
    include_doc!(concat!(stringify!($type), "/", $name))
  };
  ($outer:ident, $inner:ident, $name:literal) => {
    include_doc!(concat!(stringify!($inner), "/", stringify!($outer), "/", $name))
  };
  ($file:expr) => {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/", $file, ".md"))
  };
}

macro_rules! must_use_doc {
  () => {
    "this returns the result of the operation, without modifying the original"
  };
  ("to_ascii_uppercase") => {
    "to uppercase the value in-place, use `make_ascii_uppercase()`"
  };
  ("to_ascii_lowercase") => {
    "to lowercase the value in-place, use `make_ascii_lowercase()`"
  };
}

#[cfg(feature = "const_traits")]
macro_rules! const_trait {
  ($vis:vis const trait $($tt:tt)*) => {
    $vis const trait $($tt)*
  };
}

#[cfg(not(feature = "const_traits"))]
macro_rules! const_trait {
  ($vis:vis const trait $($tt:tt)*) => {
    $vis trait $($tt)*
  };
}

#[cfg(feature = "const_traits")]
macro_rules! const_trait_if {
  (
    $(
      #[feature($feature:literal)]
      $(#[$meta:meta])*
      impl $(<$(const $generic:ident: usize),+>)? const $trait:ident $(<$inner:ty>)? for $type:ty {
        $($body:tt)*
      }
    )*
  ) => {
    $(
      #[cfg(feature = $feature)]
      $(#[$meta])*
      impl $(<$(const $generic: usize),+>)? const $trait $(<$inner>)? for $type {
        $($body)*
      }

      #[cfg(not(feature = $feature))]
      $(#[$meta])*
      impl $(<$(const $generic: usize),+>)? $trait $(<$inner>)? for $type {
        $($body)*
      }
    )*
  };
  (
    $(
      #[feature($feature:literal)]
      $(#[$meta:meta])*
      impl<$($generic:ident: [const] $bound:ident),+> const $trait:ident $(<$inner:ty>)? for $type:ty {
        $($body:tt)*
      }
    )*
  ) => {
    $(
      #[cfg(feature = $feature)]
      $(#[$meta])*
      impl<$($generic: [const] $bound),+> const $trait $(<$inner>)? for $type {
        $($body)*
      }

      #[cfg(not(feature = $feature))]
      $(#[$meta])*
      impl<$($generic: $bound),+> $trait $(<$inner>)? for $type {
        $($body)*
      }
    )*
  };
}

#[cfg(not(feature = "const_traits"))]
macro_rules! const_trait_if {
  (
    $(
      #[feature($feature:literal)]
      $(#[$meta:meta])*
      impl $(<$(const $generic:ident: usize),+>)? const $trait:ident $(<$inner:ty>)? for $type:ty {
        $($body:tt)*
      }
    )*
  ) => {
    $(
      #[cfg(not(feature = $feature))]
      $(#[$meta])*
      impl $(<$(const $generic: usize),+>)? $trait $(<$inner>)? for $type {
        $($body)*
      }
    )*
  };
  (
    $(
      #[feature($feature:literal)]
      $(#[$meta:meta])*
      impl<$($generic:ident: [const] $bound:ident),+> const $trait:ident $(<$inner:ty>)? for $type:ty {
        $($body:tt)*
      }
    )*
  ) => {
    $(
      #[cfg(not(feature = $feature))]
      $(#[$meta])*
      impl<$($generic: $bound),+> $trait $(<$inner>)? for $type {
        $($body)*
      }
    )*
  };
}

#[cfg(feature = "const_eval_select")]
macro_rules! const_panic {
  ($ctime:literal, $rtime:literal, $($name:ident: $type:ty = $expr:expr),* $(,)?) => {{
    #[inline(always)]
    #[track_caller]
    const fn do_panic($($name: $type),*) -> ! {
      #[inline]
      #[track_caller]
      const fn ctime($($name: $type),*) -> ! {
        $(let _: $type = $name;)* // ignore unused arguments
        ::core::panic!($ctime)
      }

      #[track_caller]
      fn rtime($($name: $type),*) -> ! {
        ::core::panic!($rtime)
      }

      ::core::intrinsics::const_eval_select(($($name,)*), ctime, rtime);
    }

    do_panic($($name),*)
  }};
}

#[cfg(not(feature = "const_eval_select"))]
macro_rules! const_panic {
  ($ctime:literal, $_rtime:literal, $($_name:ident: $type:ty = $expr:expr),* $(,)?) => {{
    $(let _: $type = $expr;)* // ignore unused arguments
    ::core::panic!($ctime)
  }};
}
