use crate::llapi::Uint;

mod bigint;
mod checked;
mod general;
mod overflowing;
mod saturating;
mod strict;
mod unbounded;
mod unchecked;
mod wrapping;

/// The generic signed integer type.
///
/// ## Examples
///
/// Note that the examples here rely on the [`uint`] macro for constructing literals.
///
/// [`int<4>`]: crate::primitive::i32
/// [`uint`]: crate::uint!
#[expect(non_camel_case_types, reason = "intentional naming")]
#[repr(transparent)]
pub struct int<const N: usize = 4> {
  bytes: [u8; N],
}

// SAFETY: `int<N>` is just a byte array with no padding or uninitialized bytes.
unsafe impl<const N: usize> Uint for int<N> {}

impl<const N: usize> int<N> {
  crate::types::macros::internals!(int);
}

impl<const N: usize> int<N> {
  crate::types::macros::constants!(int);
}

impl<const N: usize> int<N> {
  crate::types::macros::binary!(int);
}

impl<const N: usize> int<N> {
  crate::types::macros::byteorder!(int);
}

impl<const N: usize> int<N> {
  crate::types::macros::parse_str!(int);
}
