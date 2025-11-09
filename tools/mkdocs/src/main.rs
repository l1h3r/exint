//! Build exint documentation from TOML files.

use aho_corasick::AhoCorasick;
use basic_toml::from_str;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::BufWriter;
use std::io::Error;
use std::io::Write;
use std::iter;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

const SINT_DOCS: &'static [&'static str] = &[
  include_str!("docs/sint/arithmetic/bigint.toml"),
  include_str!("docs/sint/arithmetic/checked.toml"),
  include_str!("docs/sint/arithmetic/general.toml"),
  include_str!("docs/sint/arithmetic/overflowing.toml"),
  include_str!("docs/sint/arithmetic/saturating.toml"),
  include_str!("docs/sint/arithmetic/strict.toml"),
  include_str!("docs/sint/arithmetic/unbounded.toml"),
  include_str!("docs/sint/arithmetic/unchecked.toml"),
  include_str!("docs/sint/arithmetic/wrapping.toml"),
  include_str!("docs/sint/binary.toml"),
  include_str!("docs/sint/byteorder.toml"),
  include_str!("docs/sint/constants.toml"),
  include_str!("docs/sint/conversion.toml"),
  include_str!("docs/sint/parse_str.toml"),
];

const UINT_DOCS: &'static [&'static str] = &[
  include_str!("docs/uint/arithmetic/bigint.toml"),
  include_str!("docs/uint/arithmetic/checked.toml"),
  include_str!("docs/uint/arithmetic/general.toml"),
  include_str!("docs/uint/arithmetic/overflowing.toml"),
  include_str!("docs/uint/arithmetic/saturating.toml"),
  include_str!("docs/uint/arithmetic/strict.toml"),
  include_str!("docs/uint/arithmetic/unbounded.toml"),
  include_str!("docs/uint/arithmetic/unchecked.toml"),
  include_str!("docs/uint/arithmetic/wrapping.toml"),
  include_str!("docs/uint/binary.toml"),
  include_str!("docs/uint/byteorder.toml"),
  include_str!("docs/uint/constants.toml"),
  include_str!("docs/uint/conversion.toml"),
  include_str!("docs/uint/extension.toml"),
  include_str!("docs/uint/parse_str.toml"),
];

const WRAPPER_DOCS: &'static [&'static str] = &[
  include_str!("docs/wrapper/binary.toml"),
  include_str!("docs/wrapper/byteorder.toml"),
  include_str!("docs/wrapper/constants.toml"),
];

const WRAPPERS: &[&str] = &[
  "Saturating",
  "Wrapping",
];

const STRICT_OVERFLOW: &str = "
# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.
";

const TOTAL_VARS: usize = 18;

// Note: This *MUST* be in the same order as `VARS`.
const KEYS: &[&str; TOTAL_VARS] = &[
  "$bits",
  "$uint_min",
  "$uint_max",
  "$int_min",
  "$int_max",
  "$to_swap",
  "$swap_b1",
  "$swap_b8",
  "$swap_be",
  "$swap_le",
  "$rotate_size",
  "$rotate_from",
  "$rotate_into",
  "$fsh_size",
  "$fsh_from",
  "$fshl_into",
  "$fshr_into",
  "$strict_overflow",
];

// Note: This *MUST* be in the same order as `KEYS`.
const VARS: &[&str; TOTAL_VARS] = &[
  "24",                 // $bits
  "0",                  // $uint_min
  "16777215",           // $uint_max
  "-8388608",           // $int_min
  "8388607",            // $int_max
  "0x123456",           // $to_swap (00010010 00110100 01010110)
  "0x563412",           // $swap_b1
  "0x6A2C48",           // $swap_b8 (01101010 00101100 01001000)
  "[0x12, 0x34, 0x56]", // $swap_be
  "[0x56, 0x34, 0x12]", // $swap_le
  "16",                 // $rotate_size
  "0x120034",           // $rotate_from
  "0x341200",           // $rotate_into
  "8",                  // $fsh_size
  "0xAABBCC",           // $fsh_from
  "0x0034AA",           // $fshl_into
  "0x34AABB",           // $fshr_into
  STRICT_OVERFLOW.trim_ascii(),
];

fn main() -> Result<(), Error> {
  let source: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
  let parent: &Path = source.parent().and_then(Path::parent).expect("parent path");
  let target: PathBuf = parent.join("src").join("docs");

  // Make sure this is a crate root
  assert!(parent.join("Cargo.toml").exists(), "missing parent Cargo.toml");

  compile_docstring(SINT_DOCS, target.join("int"), &[], &[])?;
  compile_docstring(UINT_DOCS, target.join("uint"), &[], &[])?;

  for wrapper in WRAPPERS {
    compile_docstring(WRAPPER_DOCS, target.join("int").join(wrapper), &["$outer", "$inner"], &[wrapper, "i24"])?;
    compile_docstring(WRAPPER_DOCS, target.join("uint").join(wrapper), &["$outer", "$inner"], &[wrapper, "u24"])?;
  }

  Ok(())
}

// -----------------------------------------------------------------------------
// Compile and Write Docstrings
// -----------------------------------------------------------------------------

fn compile_docstring(
  docs: &'static [&'static str],
  path: PathBuf,
  extra_keys: &[&'static str],
  extra_vars: &[&'static str],
) -> Result<(), Error> {
  fs::create_dir_all(path.as_path())?;

  let strfmt: StringFmt<'_> = StringFmt::new(extra_keys, extra_vars);

  for source in docs {
    let docs: DocFile = DocFile::parse(source)?;

    for (name, data) in docs.iter() {
      let output: PathBuf = path.join(name).with_extension("md");
      let writer: File = File::create(output)?;
      let writer: BufWriter<File> = BufWriter::new(writer);

      write_entry(writer, &strfmt, data)?;
    }
  }

  Ok(())
}

fn write_entry<W>(mut writer: W, strfmt: &StringFmt<'_>, entry: &DocEntry) -> Result<(), Error>
where
  W: Write,
{
  writeln!(writer, "{}", strfmt.format(&entry.overview))?;

  if !entry.has_examples() {
    return Ok(());
  }

  writeln!(writer)?;
  writeln!(writer, "# Examples")?;

  if let Some(ref examples) = entry.examples {
    static HEADER: &str = "Basic usage:";
    write_examples(&mut writer, HEADER, strfmt, examples, "")?;
  }

  if let Some(ref examples) = entry.examples_trailing {
    static HEADER: &str = "Trailing space returns error:";
    write_examples(&mut writer, HEADER, strfmt, examples, "")?;
  }

  if let Some(ref examples) = entry.examples_overflow {
    static HEADER: &str = "The following panics because of overflow:";
    write_examples(&mut writer, HEADER, strfmt, examples, "should_panic")?;
  }

  if let Some(ref examples) = entry.examples_div_zero {
    static HEADER: &str = "The following panics because of division by zero:";
    write_examples(&mut writer, HEADER, strfmt, examples, "should_panic")?;
  }

  if let Some(ref examples) = entry.examples_panicking {
    static HEADER: &str = "This will panic:";
    write_examples(&mut writer, HEADER, strfmt, examples, "should_panic")?;
  }

  Ok(())
}

fn write_examples<'a, W>(
  writer: &mut W,
  header: &'static str,
  strfmt: &StringFmt<'_>,
  examples: &'a DocValue,
  metadata: &'static str,
) -> Result<(), Error>
where
  W: Write,
{
  writeln!(writer)?;
  writeln!(writer, "{}", header)?;

  for example in examples.iter() {
    writeln!(writer)?;
    writeln!(writer, "```{metadata}")?;
    writeln!(writer, "# use ::exint::primitive::*;")?;
    writeln!(writer, "# ::exint::uint! {{")?;
    writeln!(writer, "{}", strfmt.format(example))?;
    writeln!(writer, "# }}")?;
    writeln!(writer, "```")?;
  }

  Ok(())
}

// -----------------------------------------------------------------------------
// String Formatter
// -----------------------------------------------------------------------------

static AHO: LazyLock<AhoCorasick> = LazyLock::new(|| {
  AhoCorasick::new(KEYS).unwrap()
});

pub struct StringFmt<'a> {
  keys: AhoCorasick,
  vars: &'a [&'static str],
}

impl<'a> StringFmt<'a> {
  pub fn new(keys: &[&'static str], vars: &'a [&'static str]) -> Self {
    assert!(
      keys.len() == vars.len(),
      "format requires a replacement for every pattern"
    );

    Self {
      keys: AhoCorasick::new(keys).unwrap(),
      vars,
    }
  }

  fn format(&self, value: impl AsRef<str>) -> String {
    let string: &str = value.as_ref().trim();
    let output: String = AHO.replace_all(string, VARS);

    if self.vars.is_empty() {
      return output;
    }

    self.keys.replace_all(output.as_str(), self.vars)
  }
}

// -----------------------------------------------------------------------------
// Doc File
// -----------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(transparent)]
pub struct DocFile {
  #[serde(borrow)]
  data: HashMap<&'static str, DocEntry>,
}

impl DocFile {
  pub fn parse(source: &'static str) -> Result<Self, Error> {
    from_str(source).map_err(Error::other)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&'static str, &DocEntry)> + '_ {
    self.data.iter().map(|(key, value)| (*key, value))
  }
}

// -----------------------------------------------------------------------------
// Doc Entry
// -----------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct DocEntry {
  #[serde(borrow)]
  overview: Cow<'static, str>,
  examples: Option<DocValue>,
  examples_trailing: Option<DocValue>,
  examples_overflow: Option<DocValue>,
  examples_div_zero: Option<DocValue>,
  examples_panicking: Option<DocValue>,
}

impl DocEntry {
  pub const fn has_examples(&self) -> bool {
    self.examples.is_some()
      || self.examples_overflow.is_some()
      || self.examples_div_zero.is_some()
      || self.examples_panicking.is_some()
  }
}

// -----------------------------------------------------------------------------
// Doc Value
// -----------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DocValue {
  #[serde(borrow)]
  Item(Cow<'static, str>),
  #[serde(borrow)]
  List(Vec<Cow<'static, str>>),
}

impl DocValue {
  pub fn iter(&self) -> Box<dyn Iterator<Item = &str> + '_> {
    match self {
      Self::Item(item) => Box::new(iter::once(item.as_ref())),
      Self::List(list) => Box::new(list.iter().map(AsRef::as_ref)),
    }
  }
}
