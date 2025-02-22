use aho_corasick::AhoCorasick;
use basic_toml::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Error;
use std::io::Write;
use std::iter;
use std::str;
use std::sync::LazyLock;

// -----------------------------------------------------------------------------
// Variables
// -----------------------------------------------------------------------------

const EXAMPLE_PRELUDE: &str = "
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
";

const STRICT_OVERFLOW: &str = "
# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.
";

mod vars {
  pub const SIZE_BITS: &str = "32";
  pub const TO_SWAP: &str = "0x12345678";
  pub const SWAPPED: &str = "0x78563412";
  pub const SWAP_BE: &str = "[0x12, 0x34, 0x56, 0x78]";
  pub const SWAP_LE: &str = "[0x78, 0x56, 0x34, 0x12]";
  pub const REVERSE: &str = "0x1E6A2C48";
  pub const ROTATE_SIZE: &str = "16";
  pub const ROTATE_FROM: &str = "0x12003400";
  pub const ROTATE_INTO: &str = "0x34001200";
  pub const STRICT_OVERFLOW: &str = super::trim(super::STRICT_OVERFLOW);
}

static AHO: LazyLock<AhoCorasick> =
  LazyLock::new(|| AhoCorasick::builder().build(Variables::KEYS).unwrap());

struct Variables;

impl Variables {
  const SIZE: usize = 12;

  const KEYS: &'static [&'static str; Self::SIZE] = &[
    "%MIN_VALUE%",
    "%MAX_VALUE%",
    "%SIZE_BITS%",
    "%TO_SWAP%",
    "%SWAPPED%",
    "%SWAP_BE%",
    "%SWAP_LE%",
    "%REVERSE%",
    "%ROTATE_SIZE%",
    "%ROTATE_FROM%",
    "%ROTATE_INTO%",
    "%STRICT_OVERFLOW%",
  ];

  // Note: This *MUST* be in the same order as `KEYS`.
  const UINT: &'static [&'static str; Self::SIZE] = &[
    "0",
    "4294967295",
    self::vars::SIZE_BITS,
    self::vars::TO_SWAP,
    self::vars::SWAPPED,
    self::vars::SWAP_BE,
    self::vars::SWAP_LE,
    self::vars::REVERSE,
    self::vars::ROTATE_SIZE,
    self::vars::ROTATE_FROM,
    self::vars::ROTATE_INTO,
    self::vars::STRICT_OVERFLOW,
  ];

  // Note: This *MUST* be in the same order as `KEYS`.
  const SINT: &'static [&'static str; Self::SIZE] = &[
    "-2147483648",
    "2147483647",
    self::vars::SIZE_BITS,
    self::vars::TO_SWAP,
    self::vars::SWAPPED,
    self::vars::SWAP_BE,
    self::vars::SWAP_LE,
    self::vars::REVERSE,
    self::vars::ROTATE_SIZE,
    self::vars::ROTATE_FROM,
    self::vars::ROTATE_INTO,
    self::vars::STRICT_OVERFLOW,
  ];

  const VARS: &'static [&'static [&'static str; Self::SIZE]] = &[Self::SINT, Self::UINT];

  fn replace(string: &str, uint: bool) -> String {
    AHO.replace_all(string, Self::VARS[uint as usize])
  }
}

// -----------------------------------------------------------------------------
// Source
// -----------------------------------------------------------------------------

pub struct Source {
  uint: bool,
  data: &'static str,
}

impl Source {
  const fn new(uint: bool, data: &'static str) -> Self {
    Self { uint, data }
  }

  pub const fn uint(&self) -> bool {
    self.uint
  }

  pub const fn data(&self) -> &'static str {
    self.data
  }
}

// -----------------------------------------------------------------------------
// Source List
// -----------------------------------------------------------------------------

pub struct SourceList {
  data: &'static [Source],
}

impl SourceList {
  const UINT: Self = Self {
    data: &[
      Source::new(true, include_str!("docs/uint/arithmetic/bigint.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/checked.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/general.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/overflowing.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/saturating.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/strict.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/unbounded.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/unchecked.toml")),
      Source::new(true, include_str!("docs/uint/arithmetic/wrapping.toml")),
      Source::new(true, include_str!("docs/uint/bit_tools.toml")),
      Source::new(true, include_str!("docs/uint/byteorder.toml")),
      Source::new(true, include_str!("docs/uint/constants.toml")),
      Source::new(true, include_str!("docs/uint/conversion.toml")),
      Source::new(true, include_str!("docs/uint/parse_str.toml")),
    ],
  };

  const SINT: Self = Self {
    data: &[
      Source::new(false, include_str!("docs/sint/arithmetic/bigint.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/checked.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/general.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/overflowing.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/saturating.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/strict.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/unbounded.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/unchecked.toml")),
      Source::new(false, include_str!("docs/sint/arithmetic/wrapping.toml")),
      Source::new(false, include_str!("docs/sint/bit_tools.toml")),
      Source::new(false, include_str!("docs/sint/byteorder.toml")),
      Source::new(false, include_str!("docs/sint/constants.toml")),
      Source::new(false, include_str!("docs/sint/conversion.toml")),
      Source::new(false, include_str!("docs/sint/parse_str.toml")),
    ],
  };

  pub fn iter() -> impl Iterator<Item = &'static Source> {
    Self::UINT.data.iter().chain(Self::SINT.data)
  }
}

// -----------------------------------------------------------------------------
// Maybe List
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MaybeList<T> {
  Item(T),
  List(Vec<T>),
}

impl<T> MaybeList<T> {
  fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
    match self {
      Self::Item(item) => Box::new(iter::once(item)),
      Self::List(list) => Box::new(list.iter()),
    }
  }
}

// -----------------------------------------------------------------------------
// Docs
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct Docs {
  overview: &'static str,
  examples: Option<MaybeList<&'static str>>,
  examples_overflow: Option<MaybeList<&'static str>>,
  examples_div_zero: Option<MaybeList<&'static str>>,
  examples_panicking: Option<MaybeList<&'static str>>,
}

impl Docs {
  pub fn write_docstring<W>(&self, mut writer: W, uint: bool) -> Result<(), Error>
  where
    W: Write,
  {
    writeln!(writer, "{}", Variables::replace(self.overview.trim(), uint))?;

    if !self.has_examples() {
      return Ok(());
    }

    writeln!(writer)?;
    writeln!(writer, "# Examples")?;

    if let Some(ref examples) = self.examples {
      static HEADER: &str = "Basic usage:";
      self.write_examples(&mut writer, HEADER, examples, uint)?;
    }

    if let Some(ref examples) = self.examples_overflow {
      static HEADER: &str = "The following panics because of overflow:";
      self.write_examples_panicking(&mut writer, HEADER, examples, uint)?;
    }

    if let Some(ref examples) = self.examples_div_zero {
      static HEADER: &str = "The following panics because of division by zero:";
      self.write_examples_panicking(&mut writer, HEADER, examples, uint)?;
    }

    if let Some(ref examples) = self.examples_panicking {
      static HEADER: &str = "This will panic:";
      self.write_examples_panicking(&mut writer, HEADER, examples, uint)?;
    }

    Ok(())
  }

  fn write_examples<W>(
    &self,
    writer: &mut W,
    header: &'static str,
    examples: &MaybeList<&'static str>,
    uint: bool,
  ) -> Result<(), Error>
  where
    W: Write,
  {
    writeln!(writer)?;
    writeln!(writer, "{}", header)?;

    for example in examples.iter() {
      writeln!(writer)?;
      writeln!(writer, "```")?;
      writeln!(writer, "{}", EXAMPLE_PRELUDE.trim())?;
      writeln!(writer, "{}", Variables::replace(example.trim(), uint))?;
      writeln!(writer, "```")?;
    }

    Ok(())
  }

  fn write_examples_panicking<W>(
    &self,
    writer: &mut W,
    header: &'static str,
    examples: &MaybeList<&'static str>,
    uint: bool,
  ) -> Result<(), Error>
  where
    W: Write,
  {
    writeln!(writer)?;
    writeln!(writer, "{}", header)?;

    for example in examples.iter() {
      writeln!(writer)?;
      writeln!(writer, "```should_panic")?;
      writeln!(writer, "{}", EXAMPLE_PRELUDE.trim())?;
      writeln!(writer, "{}", Variables::replace(example.trim(), uint))?;
      writeln!(writer, "```")?;
    }

    Ok(())
  }

  fn has_examples(&self) -> bool {
    self.examples.is_some()
      || self.examples_overflow.is_some()
      || self.examples_div_zero.is_some()
      || self.examples_panicking.is_some()
  }
}

// -----------------------------------------------------------------------------
// Doc Map
// -----------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct DocMap {
  #[serde(borrow)]
  data: HashMap<&'static str, Docs>,
}

impl DocMap {
  pub fn parse(source: &'static Source) -> Result<Self, Error> {
    from_str(source.data).map_err(Error::other)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&'static str, &Docs)> {
    self.data.iter().map(|(key, value)| (*key, value))
  }
}

// -----------------------------------------------------------------------------
// Misc. Utilities
// -----------------------------------------------------------------------------

const fn trim(string: &'static str) -> &'static str {
  let mut bytes: &[u8] = string.as_bytes();

  while let [b' ' | b'\t' | b'\n', tail @ ..] = bytes {
    bytes = tail;
  }

  while let [head @ .., b' ' | b'\t' | b'\n'] = bytes {
    bytes = head;
  }

  unsafe { str::from_utf8_unchecked(bytes) }
}
