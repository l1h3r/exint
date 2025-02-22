use ::core::convert::From;
use ::core::convert::Infallible;
use ::core::error::Error;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::Result;

// -----------------------------------------------------------------------------
// TryFromIntError
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TryFromIntError(());

impl TryFromIntError {
  #[inline]
  pub(crate) const fn new() -> Self {
    Self(())
  }
}

impl Display for TryFromIntError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str("out of range integral type conversion attempted")
  }
}

impl Error for TryFromIntError {}

impl From<Infallible> for TryFromIntError {
  fn from(other: Infallible) -> Self {
    match other {}
  }
}

#[cfg(feature = "never_type")]
impl From<!> for TryFromIntError {
  fn from(other: !) -> Self {
    match other {}
  }
}

// -----------------------------------------------------------------------------
// IntErrorKind
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntErrorKind {
  Empty,
  InvalidDigit,
  PosOverflow,
  NegOverflow,
  Zero,
}

// -----------------------------------------------------------------------------
// ParseIntError
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseIntError {
  kind: IntErrorKind,
}

impl ParseIntError {
  #[inline]
  pub(crate) const fn new(kind: IntErrorKind) -> Self {
    Self { kind }
  }

  #[must_use]
  pub const fn kind(&self) -> &IntErrorKind {
    &self.kind
  }

  // This is only exposed to allow nice panic messages in the `int/uint` macros.
  #[doc(hidden)]
  #[inline]
  pub const fn as_str(&self) -> &str {
    match self.kind {
      IntErrorKind::Empty => "cannot parse integer from empty string",
      IntErrorKind::InvalidDigit => "invalid digit found in string",
      IntErrorKind::PosOverflow => "number too large to fit in target type",
      IntErrorKind::NegOverflow => "number too small to fit in target type",
      IntErrorKind::Zero => "number would be zero for non-zero type",
    }
  }
}

impl Display for ParseIntError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.as_str())
  }
}

impl Error for ParseIntError {}
