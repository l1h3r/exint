macro_rules! assert_panic {
  ($expr:expr, message = $message:ident) => {{
    let Err(error) = ::std::panic::catch_unwind(|| $expr) else {
      ::core::panic!(concat!("expected panic: ", stringify!($expr)));
    };

    let Some(error) = error.downcast_ref::<&'_ str>() else {
      ::core::panic!("expected string error");
    };

    $crate::tests::assert_eq!(*error, $crate::panic::$message);
  }};
}

pub(crate) use assert_panic;
