use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::token_stream::IntoIter;
use std::iter::Peekable;

use crate::backports::ToTokens;
use crate::error::Error;
use crate::utilities::Context;
use crate::utilities::Expand;
use crate::utilities::SpanExt;

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

#[inline]
pub(crate) fn expand(stream: TokenStream, target: TargetMode, format: OutputFormat) -> TokenStream {
  Expander::new(target, format)
    .expand(stream)
    .unwrap_or_else(Error::into_compile_error)
}

// -----------------------------------------------------------------------------
// Target Mode
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) enum TargetMode {
  IncludeStd,
  ExcludeStd,
}

// -----------------------------------------------------------------------------
// Output Format
// -----------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(crate) enum OutputFormat {
  /// Generate code in big-endian format.
  BE,
  /// Generate code in little-endian format.
  LE,
}

impl OutputFormat {
  #[cfg(target_endian = "big")]
  pub(crate) const ANY: Self = Self::BE;
  #[cfg(target_endian = "little")]
  pub(crate) const ANY: Self = Self::LE;

  #[inline]
  const fn to_method(self) -> &'static str {
    match self {
      Self::BE => "from_be_bytes",
      Self::LE => "from_le_bytes",
    }
  }
}

// -----------------------------------------------------------------------------
// Expand Token Stream
// -----------------------------------------------------------------------------

struct Expander {
  target: TargetMode,
  format: OutputFormat,
}

impl Expander {
  #[inline]
  const fn new(target: TargetMode, format: OutputFormat) -> Self {
    Self { target, format }
  }

  #[inline]
  fn literal(
    &self,
    stream: &mut TokenStream,
    input: &Literal,
    negate: bool,
  ) -> Result<bool, Error> {
    let source: String = input.to_string();
    let parser: Parser<'_> = Parser::new(source.as_str(), self.target);

    let Some(number) = parser.parse(input.span(), self.format, negate) else {
      return Ok(false);
    };

    if number.is_overflow() {
      number.into_error_stream(input)?.to_tokens(stream);
    } else {
      number.to_tokens(stream);
    }

    Ok(true)
  }
}

impl Expand for Expander {
  fn punct(&mut self, stream: &mut TokenStream, input: Context<'_, Punct>) -> Result<(), Error> {
    fn maybe_negative(token: &Punct, value: &TokenTree) -> bool {
      if token.as_char() == '-' && let TokenTree::Literal(literal) = value {
        return SpanExt::eq(&token.span().end(), &literal.span().start());
      }

      false
    }

    let (input, tokens): (Punct, &mut Peekable<IntoIter>) = input.split();

    let Some(TokenTree::Literal(value)) = tokens.next_if(|token| maybe_negative(&input, token)) else {
      input.to_tokens(stream);
      return Ok(());
    };

    if !self.literal(stream, &value, true)? {
      input.to_tokens(stream);
      value.to_tokens(stream);
    }

    Ok(())
  }

  fn value(&mut self, stream: &mut TokenStream, input: Context<'_, Literal>) -> Result<(), Error> {
    if !self.literal(stream, &input, false)? {
      input.to_tokens(stream);
    }

    Ok(())
  }
}

// -----------------------------------------------------------------------------
// Parse Literal
// -----------------------------------------------------------------------------

struct Parser<'a> {
  source: &'a [u8],
  target: TargetMode,
}

impl<'a> Parser<'a> {
  #[inline]
  const fn new(source: &'a str, target: TargetMode) -> Self {
    Self {
      source: source.as_bytes(),
      target,
    }
  }

  #[inline]
  fn parse(mut self, source: Span, format: OutputFormat, negate: bool) -> Option<Number> {
    let digits: Digits = self.parse_digits(source)?;
    let suffix: Suffix = self.parse_suffix()?;

    let expected: usize = suffix.bytes();
    let received: usize = digits.data.len();

    let mut digits: Digits = digits;

    if expected > received {
      digits.data.resize(expected, 0);
    }

    if negate {
      digits.not();
      digits.add::<true>(1);
    }

    let mut digits: Digits = digits;

    if matches!(format, OutputFormat::BE) {
      digits.data.reverse();
    }

    Some(Number::new(source, format, digits, suffix, negate))
  }

  #[inline]
  fn bump(&mut self, count: usize) {
    self.source = &self.source[count..];
  }

  #[inline]
  fn parse_digits(&mut self, span: Span) -> Option<Digits> {
    let (radix, mut index): (u8, usize) = match self.source {
      [b'0', b'b', ..] => (2, 2),
      [b'0', b'o', ..] => (8, 2),
      [b'0', b'x', ..] => (16, 2),
      [b'0'..=b'9', ..] => (10, 0),
      _ => return None,
    };

    let mut empty: bool = false;
    let mut value: Digits = Digits::new(span);

    while index < self.source.len() {
      let digit: u8 = match self.source[index] {
        byte @ b'0'..=b'9' => byte - b'0',
        byte @ b'a'..=b'f' if radix > 10 => byte - b'a' + 10,
        byte @ b'A'..=b'F' if radix > 10 => byte - b'A' + 10,
        b'_' => {
          index += 1;
          continue;
        }
        _ => {
          break;
        }
      };

      if digit >= radix {
        return None;
      }

      value.mul(radix);
      value.add::<false>(digit);
      empty = false;
      index += 1;
    }

    if empty {
      return None;
    }

    self.bump(index);

    Some(value)
  }

  #[inline]
  fn parse_suffix(&mut self) -> Option<Suffix> {
    let [name @ (Suffix::SINT | Suffix::UINT), tail @ ..] = self.source else {
      return None;
    };

    let data: &str = str::from_utf8(tail).ok()?;
    let bits: u32 = data.parse().ok()?;

    // we don't support u0
    if bits == 0 {
      return None;
    }

    // we only support bits in multiples of 8
    if bits & 7 != 0 {
      return None;
    }

    // the caller doesn't want to expand built-in types
    if matches!(self.target, TargetMode::ExcludeStd) && matches!(bits, 8 | 16 | 32 | 64 | 128) {
      return None;
    }

    self.bump(self.source.len());

    Some(Suffix::new(*name, bits))
  }
}

// -----------------------------------------------------------------------------
// Number
// -----------------------------------------------------------------------------

struct Number {
  source: Span,
  format: OutputFormat,
  digits: Digits,
  suffix: Suffix,
  negate: bool,
}

impl Number {
  #[inline]
  const fn new(
    source: Span,
    format: OutputFormat,
    digits: Digits,
    suffix: Suffix,
    negate: bool,
  ) -> Self {
    Self {
      source,
      format,
      digits,
      suffix,
      negate,
    }
  }

  #[inline]
  fn is_overflow(&self) -> bool {
    const SIGN: u8 = 1 << (u8::BITS - 1);

    let expected: usize = self.suffix.bytes();
    let received: usize = self.digits.data.len();

    if received > expected {
      return true;
    }

    if self.digits.is_zero() {
      return false;
    }

    if let Suffix::Sint(_) = self.suffix {
      return self.negate ^ (self.digits.msb() & SIGN == SIGN);
    }

    false
  }

  #[inline]
  fn into_error_stream(self, input: &Literal) -> Result<TokenStream, Error> {
    let itype: String = self.suffix.itype();
    let error: String = format!("literal out of range for `{itype}`");

    self.into_error(input, error)
  }

  #[expect(clippy::unnecessary_wraps)]
  #[cfg(feature = "proc_macro_diagnostic")]
  fn into_error(self, input: &Literal, error: String) -> Result<TokenStream, Error> {
    use proc_macro::Diagnostic;
    use proc_macro::Level;

    let itype: String = self.suffix.itype();
    let imin: String = self.suffix.imin();
    let imax: String = self.suffix.imax();
    let note: String = format!("the literal `{input}` does not fit into the type `{itype}` whose range is `{imin}..={imax}`");

    let diagnostic: Diagnostic = Diagnostic::spanned(input.span(), Level::Error, error);
    let diagnostic: Diagnostic = diagnostic.note(note);

    diagnostic.emit();

    let data: Digits = Digits::zero(input.span(), self.suffix.bytes());
    let zero: Number = Number::new(input.span(), OutputFormat::ANY, data, self.suffix, false);

    Ok(zero.into_token_stream())
  }

  #[cfg(not(feature = "proc_macro_diagnostic"))]
  fn into_error(self, input: &Literal, error: String) -> Result<TokenStream, Error> {
    Err(Error::new(error, input.span()))
  }
}

impl ToTokens for Number {
  fn to_tokens(&self, stream: &mut TokenStream) {
    let digits: TokenStream = self.digits.to_token_stream();
    let method: Ident = self.source.new_ident(self.format.to_method());

    let ident: Ident = self.source.new_ident(self.suffix.ident());
    let bytes: usize = self.suffix.bytes();

    let tokens: TokenStream = quote_spanned! {self.source =>
      ::exint::(@ident)::<(@bytes)>::(@method)((@digits))
    };

    tokens.to_tokens(stream);
  }
}

// -----------------------------------------------------------------------------
// Digits
// -----------------------------------------------------------------------------

struct Digits {
  data: Vec<u8>,
  span: Span,
}

impl Digits {
  #[inline]
  const fn new(span: Span) -> Self {
    Self {
      data: Vec::new(),
      span,
    }
  }

  fn zero(span: Span, size: usize) -> Self {
    Self {
      data: vec![0; size],
      span,
    }
  }

  #[inline]
  fn msb(&self) -> u8 {
    self.data[self.data.len() - 1]
  }

  #[inline]
  fn is_zero(&self) -> bool {
    self.data.iter().all(|digit| *digit == 0)
  }

  #[inline]
  fn not(&mut self) {
    for digit in &mut self.data {
      *digit = !*digit;
    }
  }

  #[inline]
  fn add<const WRAP: bool>(&mut self, other: u8) {
    if self.data.is_empty() {
      self.data.push(other);
      return;
    }

    let mut carry: bool;
    let mut index: usize = 1;

    (self.data[0], carry) = self.data[0].overflowing_add(other);

    while carry {
      if index == self.data.len() {
        if WRAP {
          break;
        }

        self.data.push(0);
      }

      (self.data[index], carry) = self.data[index].overflowing_add(u8::from(carry));

      index += 1;
    }
  }

  #[inline]
  fn mul(&mut self, other: u8) {
    let mut carry: u8 = 0;

    for digit in &mut self.data {
      (*digit, carry) = digit.carrying_mul(other, carry);
    }

    if carry > 0 {
      self.data.push(carry);
    }
  }
}

impl ToTokens for Digits {
  fn to_tokens(&self, stream: &mut TokenStream) {
    let tokens: TokenStream = quote_spanned! {self.span =>
      [(#(self.data.as_slice()),*)]
    };

    tokens.to_tokens(stream);
  }
}

// -----------------------------------------------------------------------------
// Suffix
// -----------------------------------------------------------------------------

enum Suffix {
  Sint(u32),
  Uint(u32),
}

impl Suffix {
  const SINT: u8 = b'i';
  const UINT: u8 = b'u';

  #[inline]
  const fn new(name: u8, bits: u32) -> Self {
    match name {
      Self::SINT => Self::Sint(bits),
      Self::UINT => Self::Uint(bits),
      _ => unreachable!(),
    }
  }

  #[inline]
  const fn bits(&self) -> u32 {
    match self {
      Self::Sint(bits) | Self::Uint(bits) => *bits,
    }
  }

  #[inline]
  const fn bytes(&self) -> usize {
    self.bits() as usize >> 3
  }

  #[inline]
  const fn name(&self) -> char {
    match self {
      Self::Sint(_) => Self::SINT as char,
      Self::Uint(_) => Self::UINT as char,
    }
  }

  #[inline]
  const fn ident(&self) -> &'static str {
    match self {
      Self::Sint(_) => "int",
      Self::Uint(_) => "uint",
    }
  }

  #[inline]
  fn itype(&self) -> String {
    match self {
      Self::Sint(bits) => format!("{}{}", Self::SINT as char, bits),
      Self::Uint(bits) => format!("{}{}", Self::UINT as char, bits),
    }
  }

  #[inline]
  fn imax(&self) -> String {
    match self {
      Self::Sint(bits) if *bits > i128::BITS => {
        format!("2^{}-1", bits - 1)
      }
      Self::Sint(bits) => {
        format!("{}", 2_i128.wrapping_pow(bits - 1).wrapping_sub(1))
      }
      Self::Uint(bits) if *bits > u128::BITS => {
        format!("2^{bits}-1")
      }
      Self::Uint(bits) => {
        format!("{}", 2_u128.wrapping_pow(*bits).wrapping_sub(1))
      }
    }
  }

  #[inline]
  fn imin(&self) -> String {
    match self {
      Self::Sint(bits) if *bits > i128::BITS => {
        format!("-2^{}", bits - 1)
      }
      Self::Sint(bits) => {
        format!("{}", (-2_i128).wrapping_pow(bits - 1))
      }
      Self::Uint(_) => {
        "0".to_owned()
      }
    }
  }
}
