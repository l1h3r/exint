use ::core::convert::From;
use ::core::convert::Infallible;
use ::core::error::Error;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::Result;

// -----------------------------------------------------------------------------
// ParseIntError
// -----------------------------------------------------------------------------

pub type ParseIntError = ();

// -----------------------------------------------------------------------------
// TryFromIntError
// -----------------------------------------------------------------------------

/// The error type returned when a checked integral type conversion fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TryFromIntError {
  unreachable: (),
}

impl TryFromIntError {
  #[inline]
  pub(crate) const fn new() -> Self {
    Self { unreachable: () }
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
