use ::core::convert::From;
use ::core::convert::Infallible;
use ::core::error::Error;
use ::core::fmt::Display;
use ::core::fmt::Formatter;
use ::core::fmt::Result;

// -----------------------------------------------------------------------------
// TryFromIntError
// -----------------------------------------------------------------------------

/// The error type returned when a checked integral type conversion fails.
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

/// Enum to store the various types of errors that can cause parsing an integer to fail.
///
/// # Examples
///
/// ```should_panic
/// # extern crate exint_integer as exint;
/// use exint::int;
///
/// if let Err(e) = int::<4>::from_str_radix("a12", 10) {
///     panic!("Failed conversion to int: {:?}", e.kind());
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IntErrorKind {
  /// Value being parsed is empty.
  ///
  /// This variant will be constructed when parsing an empty string.
  Empty,
  /// Contains an invalid digit in its context.
  ///
  /// Among other causes, this variant will be constructed when parsing a string
  /// that contains a non-ASCII char.
  ///
  /// This variant is also constructed when a `+` or `-` is misplaced within a
  /// string either on its own or in the middle of a number.
  InvalidDigit,
  /// Integer is too large to store in target integer type.
  PosOverflow,
  /// Integer is too small to store in target integer type.
  NegOverflow,
}

// -----------------------------------------------------------------------------
// ParseIntError
// -----------------------------------------------------------------------------

/// An error which can be returned when parsing an integer.
///
/// This error is used as the error type for the `from_str_radix()` functions on
/// the generic integer types, such as [`int::from_str_radix`].
///
/// # Potential causes
///
/// Among other causes, `ParseIntError` can be thrown because of leading or
/// trailing whitespace in the string e.g., when it is obtained from the
/// standard input. Using the [`str::trim()`] method ensures that no whitespace
/// remains before parsing.
///
/// # Examples
///
/// ```should_panic
/// # extern crate exint_integer as exint;
/// use exint::int;
///
/// if let Err(e) = int::<4>::from_str_radix("a12", 10) {
///     panic!("Failed conversion to int: {e}");
/// }
/// ```
///
/// [`int::from_str_radix`]: crate::int::from_str_radix
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseIntError {
  kind: IntErrorKind,
}

impl ParseIntError {
  #[inline]
  pub(crate) const fn new(kind: IntErrorKind) -> Self {
    Self { kind }
  }

  /// Outputs the detailed cause of parsing an integer failing.
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
    }
  }
}

impl Display for ParseIntError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.as_str())
  }
}

impl Error for ParseIntError {}
