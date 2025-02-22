/// The generic signed integer type.
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct int<const S: usize = 4> {
  bytes: [u8; S],
}

impl<const S: usize> int<S> {
  crate::macros::internals!(int);
}

impl<const S: usize> int<S> {
  crate::macros::byteorder!(int);
  crate::macros::constants!(int);

  crate::macros::convert!(int);
  crate::macros::inspect!(int);

  crate::macros::bigint!(int);
  crate::macros::checked!(int);
  crate::macros::generic!(int);
  crate::macros::overflowing!(int);
  crate::macros::saturating!(int);
  crate::macros::strict!(int);
  crate::macros::unbounded!(int);
  crate::macros::unchecked!(int);
  crate::macros::wrapping!(int);
}
