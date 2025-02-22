use ::core::panic;
use ::core::marker::Sized;
use ::core::option::Option;
use ::core::option::Option::None;
use ::core::option::Option::Some;
use ::core::result::Result;

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
  crate::macros::parse_str!(int);

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

crate::traits::clone!(int);
crate::traits::eq!(int);
crate::traits::ord!(int);
crate::traits::partial_eq!(int);
crate::traits::partial_ord!(int);
crate::traits::convert!(int);
crate::traits::default!(int);
crate::traits::fmt!(int);
crate::traits::hash!(int);
crate::traits::product!(int);
crate::traits::step!(int);
crate::traits::sum!(int);
crate::traits::trusted_step!(int);
crate::traits::copy!(int);
crate::traits::binops!(int);
crate::traits::unops!(int);
crate::traits::random!(int);
crate::traits::from_str!(int);
