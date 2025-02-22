use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::format::StringFmt;
use crate::format::EXAMPLE_PRELUDE;
use crate::parse::DocMap;
use crate::parse::DocStr;
use crate::parse::Value;

mod format;
mod parse;
mod utils;

// -----------------------------------------------------------------------------
// Source List
// -----------------------------------------------------------------------------

pub struct SourceList {
  data: &'static [&'static str],
}

impl SourceList {
  pub const SINT_TY: Self = Self {
    data: &[
      include_str!("docs/sint/arithmetic/bigint.toml"),
      include_str!("docs/sint/arithmetic/checked.toml"),
      include_str!("docs/sint/arithmetic/general.toml"),
      include_str!("docs/sint/arithmetic/overflowing.toml"),
      include_str!("docs/sint/arithmetic/saturating.toml"),
      include_str!("docs/sint/arithmetic/strict.toml"),
      include_str!("docs/sint/arithmetic/unbounded.toml"),
      include_str!("docs/sint/arithmetic/unchecked.toml"),
      include_str!("docs/sint/arithmetic/wrapping.toml"),
      include_str!("docs/sint/bit_tools.toml"),
      include_str!("docs/sint/byteorder.toml"),
      include_str!("docs/sint/constants.toml"),
      include_str!("docs/sint/conversion.toml"),
      include_str!("docs/sint/parse_str.toml"),
    ],
  };

  pub const UINT_TY: Self = Self {
    data: &[
      include_str!("docs/uint/arithmetic/bigint.toml"),
      include_str!("docs/uint/arithmetic/checked.toml"),
      include_str!("docs/uint/arithmetic/general.toml"),
      include_str!("docs/uint/arithmetic/overflowing.toml"),
      include_str!("docs/uint/arithmetic/saturating.toml"),
      include_str!("docs/uint/arithmetic/strict.toml"),
      include_str!("docs/uint/arithmetic/unbounded.toml"),
      include_str!("docs/uint/arithmetic/unchecked.toml"),
      include_str!("docs/uint/arithmetic/wrapping.toml"),
      include_str!("docs/uint/bit_tools.toml"),
      include_str!("docs/uint/byteorder.toml"),
      include_str!("docs/uint/constants.toml"),
      include_str!("docs/uint/conversion.toml"),
      include_str!("docs/uint/extension.toml"),
      include_str!("docs/uint/parse_str.toml"),
    ],
  };

  pub const WRAPPER: Self = Self {
    data: &[
      include_str!("docs/wrapper/bit_tools.toml"),
      include_str!("docs/wrapper/byteorder.toml"),
      include_str!("docs/wrapper/constants.toml"),
    ],
  };

  pub fn build_base_ty<P>(&self, path: P) -> Result<(), Error>
  where
    P: AsRef<Path>,
  {
    self.compile_docstring(path.as_ref(), &[], &[])
  }

  pub fn build_wrapper<P>(
    &self,
    path: P,
    outer: &'static str,
    inner: &'static str,
  ) -> Result<(), Error>
  where
    P: AsRef<Path>,
  {
    self.compile_docstring(path.as_ref(), &["$outer", "$inner"], &[outer, inner])
  }

  fn compile_docstring(
    &self,
    path: &Path,
    extra_keys: &[&'static str],
    extra_vars: &[&'static str],
  ) -> Result<(), Error> {
    fs::create_dir_all(path)?;

    for source in self.data {
      let docs: DocMap = DocMap::parse(source)?;

      for (name, data) in docs.iter() {
        let output: PathBuf = path.join(name).with_extension("md");
        let writer: File = File::create(output)?;
        let writer: BufWriter<File> = BufWriter::new(writer);

        write_docstring(writer, data, extra_keys, extra_vars)?;
      }
    }

    Ok(())
  }
}

fn write_docstring<W>(
  mut writer: W,
  doc_str: &DocStr,
  extra_keys: &[&'static str],
  extra_vars: &[&'static str],
) -> Result<(), Error>
where
  W: Write,
{
  let mut formatter: StringFmt<'_> = StringFmt::new(doc_str.overview(), extra_keys, extra_vars);

  writeln!(writer, "{}", formatter)?;

  if !doc_str.has_examples() {
    return Ok(());
  }

  writeln!(writer)?;
  writeln!(writer, "# Examples")?;

  if let Some(examples) = doc_str.examples() {
    // TODO: skip header for some methods (u8 exts, u16 exts)
    static HEADER: &str = "Basic usage:";
    write_examples(&mut writer, HEADER, examples, &mut formatter)?;
  }

  if let Some(examples) = doc_str.examples_trailing() {
    static HEADER: &str = "Trailing space returns error:";
    write_examples(&mut writer, HEADER, examples, &mut formatter)?;
  }

  if let Some(examples) = doc_str.examples_overflow() {
    static HEADER: &str = "The following panics because of overflow:";
    write_examples_panicking(&mut writer, HEADER, examples, &mut formatter)?;
  }

  if let Some(examples) = doc_str.examples_div_zero() {
    static HEADER: &str = "The following panics because of division by zero:";
    write_examples_panicking(&mut writer, HEADER, examples, &mut formatter)?;
  }

  if let Some(examples) = doc_str.examples_panicking() {
    static HEADER: &str = "This will panic:";
    write_examples_panicking(&mut writer, HEADER, examples, &mut formatter)?;
  }

  Ok(())
}

fn write_examples<'a, W>(
  writer: &mut W,
  header: &'static str,
  examples: &'a Value,
  formatter: &mut StringFmt<'a>,
) -> Result<(), Error>
where
  W: Write,
{
  writeln!(writer)?;
  writeln!(writer, "{}", header)?;

  for example in examples.iter() {
    formatter.set_value(example);
    writeln!(writer)?;
    writeln!(writer, "```")?;
    writeln!(writer, "{}", EXAMPLE_PRELUDE.trim())?;
    writeln!(writer, "{}", formatter)?;
    writeln!(writer, "```")?;
  }

  Ok(())
}

fn write_examples_panicking<'a, W>(
  writer: &mut W,
  header: &'static str,
  examples: &'a Value,
  formatter: &mut StringFmt<'a>,
) -> Result<(), Error>
where
  W: Write,
{
  writeln!(writer)?;
  writeln!(writer, "{}", header)?;

  for example in examples.iter() {
    formatter.set_value(example);
    writeln!(writer)?;
    writeln!(writer, "```should_panic")?;
    writeln!(writer, "{}", EXAMPLE_PRELUDE.trim())?;
    writeln!(writer, "{}", formatter)?;
    writeln!(writer, "```")?;
  }

  Ok(())
}
