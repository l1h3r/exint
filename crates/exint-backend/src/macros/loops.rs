/// Add limited 'for loop'-style support in const functions.
macro_rules! cfor {
  (@for $index:ident in $lower:tt..$upper:tt $block:block) => {{
    let mut $index: usize = $lower;

    while $index < $upper {
      $block
      $index += 1;
    }
  }};
}

/// A [`cfor`] wrapper to map an integer byte array in const functions.
macro_rules! cmap {
  (@for $index:ident in $lower:tt..$upper:tt do $expr:expr) => {{
    let mut data: $crate::types::Int<$upper> = [0; $upper];

    $crate::macros::cfor! {
      @for $index in $lower..$upper {
        data[$index] = $expr;
      }
    }

    data
  }};
}

/// A [`cfor`] wrapper to reverse map an integer byte array in const functions.
macro_rules! crev {
  (@for $index:ident in $lower:tt..$upper:tt do $expr:expr) => {{
    let mut data: $crate::types::Int<$upper> = [0; $upper];

    $crate::macros::cfor! {
      @for $index in $lower..$upper {
        data[$upper - $index - 1] = $expr;
      }
    }

    data
  }};
}

pub(crate) use cfor;
pub(crate) use cmap;
pub(crate) use crev;
