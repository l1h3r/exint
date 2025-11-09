macro_rules! assert_shift {
  (exact_shl, $name:ident, $value:expr, $a:tt) => {
    $crate::tests::assert_shift!(<<, exact_shl, $name, $value, $a, $a, $a, $a, $a, $a, $a);
  };
  (exact_shl, $name:ident, $value:expr, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => {
    $crate::tests::assert_shift!(<<, exact_shl, $name, $value, $a, $b, $c, $d, $e, $f, $g);
  };
  (exact_shr, $name:ident, $value:expr, $a:tt) => {
    $crate::tests::assert_shift!(>>, exact_shr, $name, $value, $a, $a, $a, $a, $a, $a, $a);
  };
  (exact_shr, $name:ident, $value:expr, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => {
    $crate::tests::assert_shift!(>>, exact_shr, $name, $value, $a, $b, $c, $d, $e, $f, $g);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr, $a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt) => {{
    #[allow(unused_macro_rules, reason = "macro-generated")]
    macro_rules! arg {
      (None) => {
        None
      };
      ($item:literal) => {
        Some($value $operator $item)
      };
      ($item:ident) => {
        Some($value $operator $name::$item)
      };
    }

    $crate::tests::assert_eq!($value.$method(0), Some($value));
    $crate::tests::assert_eq!($value.$method(1), arg!($a));
    $crate::tests::assert_eq!($value.$method(3), arg!($b));
    $crate::tests::assert_eq!($value.$method(5), arg!($c));

    $crate::tests::assert_eq!($value.$method($name::S_1), arg!($d));
    $crate::tests::assert_eq!($value.$method($name::S_2), arg!($e));
    $crate::tests::assert_eq!($value.$method($name::S_3), arg!($f));
    $crate::tests::assert_eq!($value.$method($name::S_4), arg!($g));

    $crate::tests::assert_eq!($value.$method($name::BITS + 1), None);
    $crate::tests::assert_eq!($value.$method($name::BITS + 3), None);
    $crate::tests::assert_eq!($value.$method($name::BITS + 5), None);

    $crate::tests::assert_eq!($value.$method($name::BITS), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 1), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 3), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 5), None);
  }};
}

macro_rules! assert_checked_shift {
  (checked_shl, $name:ident, $value:expr) => {
    $crate::tests::assert_checked_shift!(<<, checked_shl, $name, $value);
  };
  (checked_shr, $name:ident, $value:expr) => {
    $crate::tests::assert_checked_shift!(>>, checked_shr, $name, $value);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr) => {{
    $crate::tests::assert_eq!($value.$method(0), Some($value));
    $crate::tests::assert_eq!($value.$method(1), Some($value $operator 1));
    $crate::tests::assert_eq!($value.$method(3), Some($value $operator 3));
    $crate::tests::assert_eq!($value.$method(5), Some($value $operator 5));

    $crate::tests::assert_eq!($value.$method($name::S_1), Some($value $operator $name::S_1));
    $crate::tests::assert_eq!($value.$method($name::S_2), Some($value $operator $name::S_2));
    $crate::tests::assert_eq!($value.$method($name::S_3), Some($value $operator $name::S_3));
    $crate::tests::assert_eq!($value.$method($name::S_4), Some($value $operator $name::S_4));

    $crate::tests::assert_eq!($value.$method($name::BITS + 1), None);
    $crate::tests::assert_eq!($value.$method($name::BITS + 3), None);
    $crate::tests::assert_eq!($value.$method($name::BITS + 5), None);

    $crate::tests::assert_eq!($value.$method($name::BITS), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 1), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 3), None);
    $crate::tests::assert_eq!($value.$method($name::BITS << 5), None);
  }};
}

macro_rules! assert_overflowing_shift {
  (overflowing_shl, $name:ident, $value:expr) => {
    $crate::tests::assert_overflowing_shift!(<<, overflowing_shl, $name, $value);
  };
  (overflowing_shr, $name:ident, $value:expr) => {
    $crate::tests::assert_overflowing_shift!(>>, overflowing_shr, $name, $value);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr) => {{
    $crate::tests::assert_eq!($value.$method(0), ($value, false));
    $crate::tests::assert_eq!($value.$method(1), ($value $operator 1, false));
    $crate::tests::assert_eq!($value.$method(3), ($value $operator 3, false));
    $crate::tests::assert_eq!($value.$method(5), ($value $operator 5, false));

    $crate::tests::assert_eq!($value.$method($name::S_1), ($value $operator $name::S_1, false));
    $crate::tests::assert_eq!($value.$method($name::S_2), ($value $operator $name::S_2, false));
    $crate::tests::assert_eq!($value.$method($name::S_3), ($value $operator $name::S_3, false));
    $crate::tests::assert_eq!($value.$method($name::S_4), ($value $operator $name::S_4, false));

    $crate::tests::assert_eq!($value.$method($name::BITS + 1), ($value $operator 1, true));
    $crate::tests::assert_eq!($value.$method($name::BITS + 3), ($value $operator 3, true));
    $crate::tests::assert_eq!($value.$method($name::BITS + 5), ($value $operator 5, true));

    $crate::tests::assert_eq!($value.$method($name::BITS), ($value, true));
    $crate::tests::assert_eq!($value.$method($name::BITS << 1), ($value, true));
    $crate::tests::assert_eq!($value.$method($name::BITS << 3), ($value, true));
    $crate::tests::assert_eq!($value.$method($name::BITS << 5), ($value, true));
  }};
}

macro_rules! assert_strict_shift {
  (strict_shl, $name:ident, $value:expr) => {
    $crate::tests::assert_strict_shift!(<<, strict_shl, $name, $value, SHL);
  };
  (strict_shr, $name:ident, $value:expr) => {
    $crate::tests::assert_strict_shift!(>>, strict_shr, $name, $value, SHR);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr, $tag:ident) => {{
    $crate::tests::assert_eq!($value.$method(0), $value);
    $crate::tests::assert_eq!($value.$method(1), $value $operator 1);
    $crate::tests::assert_eq!($value.$method(3), $value $operator 3);
    $crate::tests::assert_eq!($value.$method(5), $value $operator 5);

    $crate::tests::assert_eq!($value.$method($name::S_1), $value $operator $name::S_1);
    $crate::tests::assert_eq!($value.$method($name::S_2), $value $operator $name::S_2);
    $crate::tests::assert_eq!($value.$method($name::S_3), $value $operator $name::S_3);
    $crate::tests::assert_eq!($value.$method($name::S_4), $value $operator $name::S_4);

    $crate::tests::assert_panic!($value.$method($name::BITS + 1), message = $tag);
    $crate::tests::assert_panic!($value.$method($name::BITS + 3), message = $tag);
    $crate::tests::assert_panic!($value.$method($name::BITS + 5), message = $tag);

    $crate::tests::assert_panic!($value.$method($name::BITS), message = $tag);
    $crate::tests::assert_panic!($value.$method($name::BITS << 1), message = $tag);
    $crate::tests::assert_panic!($value.$method($name::BITS << 3), message = $tag);
    $crate::tests::assert_panic!($value.$method($name::BITS << 5), message = $tag);
  }};
}

macro_rules! assert_unbounded_shift {
  (unbounded_shl, $name:ident, $value:expr, $overflow:expr) => {
    $crate::tests::assert_unbounded_shift!(<<, unbounded_shl, $name, $value, $overflow);
  };
  (unbounded_shr, $name:ident, $value:expr, $overflow:expr) => {
    $crate::tests::assert_unbounded_shift!(>>, unbounded_shr, $name, $value, $overflow);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr, $overflow:expr) => {{
    $crate::tests::assert_eq!($value.$method(0), $value);
    $crate::tests::assert_eq!($value.$method(1), $value $operator 1);
    $crate::tests::assert_eq!($value.$method(3), $value $operator 3);
    $crate::tests::assert_eq!($value.$method(5), $value $operator 5);

    $crate::tests::assert_eq!($value.$method($name::S_1), $value $operator $name::S_1);
    $crate::tests::assert_eq!($value.$method($name::S_2), $value $operator $name::S_2);
    $crate::tests::assert_eq!($value.$method($name::S_3), $value $operator $name::S_3);
    $crate::tests::assert_eq!($value.$method($name::S_4), $value $operator $name::S_4);

    $crate::tests::assert_eq!($value.$method($name::BITS + 1), $overflow);
    $crate::tests::assert_eq!($value.$method($name::BITS + 3), $overflow);
    $crate::tests::assert_eq!($value.$method($name::BITS + 5), $overflow);

    $crate::tests::assert_eq!($value.$method($name::BITS), $overflow);
    $crate::tests::assert_eq!($value.$method($name::BITS << 1), $overflow);
    $crate::tests::assert_eq!($value.$method($name::BITS << 3), $overflow);
    $crate::tests::assert_eq!($value.$method($name::BITS << 5), $overflow);
  }};
}

macro_rules! assert_wrapping_shift {
  (wrapping_shl, $name:ident, $value:expr) => {
    $crate::tests::assert_wrapping_shift!(<<, wrapping_shl, $name, $value);
  };
  (wrapping_shr, $name:ident, $value:expr) => {
    $crate::tests::assert_wrapping_shift!(>>, wrapping_shr, $name, $value);
  };
  ($operator:tt, $method:ident, $name:ident, $value:expr) => {{
    $crate::tests::assert_eq!($value.$method(0), $value);
    $crate::tests::assert_eq!($value.$method(1), $value $operator 1);
    $crate::tests::assert_eq!($value.$method(3), $value $operator 3);
    $crate::tests::assert_eq!($value.$method(5), $value $operator 5);

    $crate::tests::assert_eq!($value.$method($name::S_1), $value $operator $name::S_1);
    $crate::tests::assert_eq!($value.$method($name::S_2), $value $operator $name::S_2);
    $crate::tests::assert_eq!($value.$method($name::S_3), $value $operator $name::S_3);
    $crate::tests::assert_eq!($value.$method($name::S_4), $value $operator $name::S_4);

    $crate::tests::assert_eq!($value.$method($name::BITS + 1), $value $operator 1);
    $crate::tests::assert_eq!($value.$method($name::BITS + 3), $value $operator 3);
    $crate::tests::assert_eq!($value.$method($name::BITS + 5), $value $operator 5);

    $crate::tests::assert_eq!($value.$method($name::BITS), $value);
    $crate::tests::assert_eq!($value.$method($name::BITS << 1), $value);
    $crate::tests::assert_eq!($value.$method($name::BITS << 3), $value);
    $crate::tests::assert_eq!($value.$method($name::BITS << 5), $value);
  }};
}

pub(crate) use assert_checked_shift;
pub(crate) use assert_overflowing_shift;
pub(crate) use assert_shift;
pub(crate) use assert_strict_shift;
pub(crate) use assert_unbounded_shift;
pub(crate) use assert_wrapping_shift;
