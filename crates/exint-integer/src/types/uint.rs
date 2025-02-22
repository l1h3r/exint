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
  crate::macros::parse_str!(uint);

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

crate::traits::clone!(uint);
crate::traits::eq!(uint);
crate::traits::ord!(uint);
crate::traits::partial_eq!(uint);
crate::traits::partial_ord!(uint);
crate::traits::convert!(uint);
crate::traits::default!(uint);
crate::traits::fmt!(uint);
crate::traits::hash!(uint);
crate::traits::product!(uint);
crate::traits::step!(uint);
crate::traits::sum!(uint);
crate::traits::trusted_step!(uint);
crate::traits::copy!(uint);
crate::traits::binops!(uint);
crate::traits::unops!(uint);
crate::traits::random!(uint);
crate::traits::from_str!(uint);
