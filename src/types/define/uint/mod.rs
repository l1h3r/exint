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

/// The generic unsigned integer type.
///
/// ## Examples
///
/// Note that the examples here rely on the [`uint`] macro for constructing literals.
///
/// [`uint<4>`]: crate::primitive::u32
/// [`uint`]: crate::uint!
#[expect(non_camel_case_types, reason = "intentional naming")]
#[repr(transparent)]
pub struct uint<const N: usize = 4> {
  bytes: [u8; N],
}

// SAFETY: `uint<N>` is just a byte array with no padding or uninitialized bytes.
unsafe impl<const N: usize> Uint for uint<N> {}

impl<const N: usize> uint<N> {
  crate::types::macros::internals!(uint);
}

impl<const N: usize> uint<N> {
  crate::types::macros::constants!(uint);
}

impl<const N: usize> uint<N> {
  crate::types::macros::binary!(uint);
}

impl<const N: usize> uint<N> {
  crate::types::macros::byteorder!(uint);
}

impl<const N: usize> uint<N> {
  crate::types::macros::parse_str!(uint);
}

// -----------------------------------------------------------------------------
// u8 Extensions
// -----------------------------------------------------------------------------

impl uint<1> {
  #[doc = include_doc!(uint, "is_ascii")]
  #[must_use]
  #[inline]
  pub const fn is_ascii(&self) -> bool {
    self.into_u8().is_ascii()
  }

  #[doc = include_doc!(uint, "is_ascii_alphabetic")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_alphabetic(&self) -> bool {
    self.into_u8().is_ascii_alphabetic()
  }

  #[doc = include_doc!(uint, "is_ascii_alphanumeric")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_alphanumeric(&self) -> bool {
    self.into_u8().is_ascii_alphanumeric()
  }

  #[doc = include_doc!(uint, "is_ascii_digit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_digit(&self) -> bool {
    self.into_u8().is_ascii_digit()
  }

  #[doc = include_doc!(uint, "is_ascii_uppercase")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_uppercase(&self) -> bool {
    self.into_u8().is_ascii_uppercase()
  }

  #[doc = include_doc!(uint, "is_ascii_lowercase")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_lowercase(&self) -> bool {
    self.into_u8().is_ascii_lowercase()
  }

  #[doc = include_doc!(uint, "is_ascii_hexdigit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_hexdigit(&self) -> bool {
    self.into_u8().is_ascii_hexdigit()
  }

  #[doc = include_doc!(uint, "is_ascii_octdigit")]
  #[cfg(feature = "is_ascii_octdigit")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_octdigit(&self) -> bool {
    ::core::matches!(self.into_u8(), b'0'..=b'7')
  }

  #[doc = include_doc!(uint, "is_ascii_punctuation")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_punctuation(&self) -> bool {
    self.into_u8().is_ascii_punctuation()
  }

  #[doc = include_doc!(uint, "is_ascii_graphic")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_graphic(&self) -> bool {
    self.into_u8().is_ascii_graphic()
  }

  #[doc = include_doc!(uint, "is_ascii_whitespace")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_whitespace(&self) -> bool {
    self.into_u8().is_ascii_whitespace()
  }

  #[doc = include_doc!(uint, "is_ascii_control")]
  #[must_use]
  #[inline]
  pub const fn is_ascii_control(&self) -> bool {
    self.into_u8().is_ascii_control()
  }

  #[doc = include_doc!(uint, "as_ascii")]
  #[cfg(feature = "ascii_char")]
  #[must_use]
  #[inline]
  pub const fn as_ascii(&self) -> ::core::option::Option<::core::ascii::Char> {
    self.into_u8().as_ascii()
  }

  #[doc = include_doc!(uint, "as_ascii_unchecked")]
  #[cfg(feature = "ascii_char")]
  #[must_use]
  #[inline]
  pub const unsafe fn as_ascii_unchecked(&self) -> ::core::ascii::Char {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { self.into_u8().as_ascii_unchecked() }
  }

  #[doc = include_doc!(uint, "to_ascii_uppercase")]
  #[must_use = must_use_doc!("to_ascii_uppercase")]
  #[inline]
  pub const fn to_ascii_uppercase(&self) -> uint<1> {
    Self::from_u8(self.into_u8().to_ascii_uppercase())
  }

  #[doc = include_doc!(uint, "to_ascii_lowercase")]
  #[must_use = must_use_doc!("to_ascii_lowercase")]
  #[inline]
  pub const fn to_ascii_lowercase(&self) -> uint<1> {
    Self::from_u8(self.into_u8().to_ascii_lowercase())
  }

  #[doc = include_doc!(uint, "make_ascii_uppercase")]
  #[inline]
  pub const fn make_ascii_uppercase(&mut self) {
    *self = self.to_ascii_uppercase();
  }

  #[doc = include_doc!(uint, "make_ascii_lowercase")]
  #[inline]
  pub const fn make_ascii_lowercase(&mut self) {
    *self = self.to_ascii_lowercase();
  }

  #[doc = include_doc!(uint, "eq_ignore_ascii_case")]
  #[inline]
  pub const fn eq_ignore_ascii_case(&self, rhs: &Self) -> bool {
    self.into_u8().eq_ignore_ascii_case(&rhs.into_u8())
  }

  #[doc = include_doc!(uint, "escape_ascii")]
  #[must_use = must_use_doc!()]
  #[inline]
  pub fn escape_ascii(self) -> ::core::ascii::EscapeDefault {
    self.into_u8().escape_ascii()
  }
}

// -----------------------------------------------------------------------------
// u16 Extensions
// -----------------------------------------------------------------------------

impl uint<2> {
  #[doc = include_doc!(uint, "is_utf16_surrogate")]
  #[cfg(feature = "utf16_extra")]
  #[must_use]
  #[inline]
  pub const fn is_utf16_surrogate(self) -> bool {
    ::core::matches!(self.into_u16(), 0xD800..=0xDFFF)
  }
}
