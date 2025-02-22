/// The generic unsigned integer type.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct uint<const S: usize = 4> {
  bytes: [u8; S],
}

impl<const S: usize> uint<S> {
  crate::macros::internals!(uint);
}

impl<const S: usize> uint<S> {
  crate::macros::byteorder!(uint);
  crate::macros::constants!(uint);

  crate::macros::convert!(uint);
  crate::macros::inspect!(uint);

  crate::macros::bigint!(uint);
  crate::macros::checked!(uint);
  crate::macros::generic!(uint);
  crate::macros::overflowing!(uint);
  crate::macros::saturating!(uint);
  crate::macros::strict!(uint);
  crate::macros::unbounded!(uint);
  crate::macros::unchecked!(uint);
  crate::macros::wrapping!(uint);
}
