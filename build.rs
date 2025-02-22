//! Build exint documentation from TOML files.
// TODO: Move everything to separate crate with script (alias to cargo mkdocs)

use basic_toml::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Error as FmtError;
use std::fmt::Write;
use std::fs;
use std::iter;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

type Vars = HashMap<&'static str, &'static str>;

const UINT_SOURCES: &[&str] = &[
  include_str!("src/docs/uint/arithmetic/bigint.toml"),
  include_str!("src/docs/uint/arithmetic/checked.toml"),
  include_str!("src/docs/uint/arithmetic/general.toml"),
  include_str!("src/docs/uint/arithmetic/overflowing.toml"),
  include_str!("src/docs/uint/arithmetic/saturating.toml"),
  include_str!("src/docs/uint/arithmetic/strict.toml"),
  include_str!("src/docs/uint/arithmetic/unbounded.toml"),
  include_str!("src/docs/uint/arithmetic/unchecked.toml"),
  include_str!("src/docs/uint/arithmetic/wrapping.toml"),
  include_str!("src/docs/uint/assorted.toml"),
  include_str!("src/docs/uint/bit_tools.toml"),
  include_str!("src/docs/uint/byteorder.toml"),
  include_str!("src/docs/uint/constants.toml"),
  include_str!("src/docs/uint/parse_str.toml"),
];

const SINT_SOURCES: &[&str] = &[
  include_str!("src/docs/sint/arithmetic/bigint.toml"),
  include_str!("src/docs/sint/arithmetic/checked.toml"),
  include_str!("src/docs/sint/arithmetic/general.toml"),
  include_str!("src/docs/sint/arithmetic/overflowing.toml"),
  include_str!("src/docs/sint/arithmetic/saturating.toml"),
  include_str!("src/docs/sint/arithmetic/strict.toml"),
  include_str!("src/docs/sint/arithmetic/unbounded.toml"),
  include_str!("src/docs/sint/arithmetic/unchecked.toml"),
  include_str!("src/docs/sint/arithmetic/wrapping.toml"),
  include_str!("src/docs/sint/assorted.toml"),
  include_str!("src/docs/sint/bit_tools.toml"),
  include_str!("src/docs/sint/byteorder.toml"),
  include_str!("src/docs/sint/constants.toml"),
  include_str!("src/docs/sint/parse_str.toml"),
];

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

static CORE_VARS: LazyLock<Vars> = LazyLock::new(|| {
  let mut this: Vars = Vars::new();
  let _: Option<_> = this.insert("%SIZE_BITS%", "32");
  let _: Option<_> = this.insert("%TO_SWAP%", "0x12345678");
  let _: Option<_> = this.insert("%SWAPPED%", "0x78563412");
  let _: Option<_> = this.insert("%SWAP_BE%", "[0x12, 0x34, 0x56, 0x78]");
  let _: Option<_> = this.insert("%SWAP_LE%", "[0x78, 0x56, 0x34, 0x12]");
  let _: Option<_> = this.insert("%REVERSE%", "0x1E6A2C48");
  let _: Option<_> = this.insert("%ROTATE_SIZE%", "16");
  let _: Option<_> = this.insert("%ROTATE_FROM%", "0x12003400");
  let _: Option<_> = this.insert("%ROTATE_INTO%", "0x34001200");
  let _: Option<_> = this.insert("%STRICT_OVERFLOW%", STRICT_OVERFLOW.trim());
  this
});

static UINT_VARS: LazyLock<Vars> = LazyLock::new(|| {
  let mut this: Vars = CORE_VARS.clone();
  let _: Option<_> = this.insert("%MIN_VALUE%", "0");
  let _: Option<_> = this.insert("%MAX_VALUE%", "4294967295");
  this
});

static SINT_VARS: LazyLock<Vars> = LazyLock::new(|| {
  let mut this: Vars = CORE_VARS.clone();
  let _: Option<_> = this.insert("%MIN_VALUE%", "-2147483648");
  let _: Option<_> = this.insert("%MAX_VALUE%", "2147483647");
  this
});

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

#[derive(Debug, Deserialize)]
struct Docs<const UINT: bool> {
  overview: &'static str,
  examples: Option<&'static str>,
  examples_overflow: Option<MaybeList<&'static str>>,
  examples_div_zero: Option<MaybeList<&'static str>>,
  examples_panicking: Option<MaybeList<&'static str>>,
}

impl<const UINT: bool> Docs<UINT> {
  fn has_examples(&self) -> bool {
    self.examples.is_some()
      || self.examples_overflow.is_some()
      || self.examples_div_zero.is_some()
      || self.examples_panicking.is_some()
  }

  fn variables(&self) -> &'static Vars {
    if UINT {
      &UINT_VARS
    } else {
      &SINT_VARS
    }
  }

  fn replace(&self, string: &str) -> String {
    self
      .variables()
      .iter()
      .fold(string.to_owned(), |acc, (key, val)| acc.replace(key, val))
  }

  fn to_docstring(&self) -> Result<String, FmtError> {
    let mut buffer: String = String::new();

    writeln!(buffer, "{}", self.replace(self.overview.trim()))?;

    if self.has_examples() {
      writeln!(buffer)?;
      writeln!(buffer, "# Examples")?;
      writeln!(buffer)?;
      writeln!(buffer, "Basic usage:")?;
    }

    if let Some(examples) = self.examples {
      writeln!(buffer)?;
      writeln!(buffer, "```")?;
      writeln!(buffer, "{}", EXAMPLE_PRELUDE.trim())?;
      writeln!(buffer, "{}", self.replace(examples.trim()))?;
      writeln!(buffer, "```")?;
    }

    if let Some(ref examples) = self.examples_overflow {
      writeln!(buffer)?;
      writeln!(buffer, "The following panics because of overflow:")?;

      for example in examples.iter() {
        writeln!(buffer)?;
        writeln!(buffer, "```should_panic")?;
        writeln!(buffer, "{}", EXAMPLE_PRELUDE.trim())?;
        writeln!(buffer, "{}", self.replace(example.trim()))?;
        writeln!(buffer, "```")?;
      }
    }

    if let Some(ref examples) = self.examples_div_zero {
      writeln!(buffer)?;
      writeln!(buffer, "The following panics because of division by zero:")?;

      for example in examples.iter() {
        writeln!(buffer)?;
        writeln!(buffer, "```should_panic")?;
        writeln!(buffer, "{}", EXAMPLE_PRELUDE.trim())?;
        writeln!(buffer, "{}", self.replace(example.trim()))?;
        writeln!(buffer, "```")?;
      }
    }

    if let Some(ref examples) = self.examples_panicking {
      writeln!(buffer)?;
      writeln!(buffer, "This will panic:")?;

      for example in examples.iter() {
        writeln!(buffer)?;
        writeln!(buffer, "```should_panic")?;
        writeln!(buffer, "{}", EXAMPLE_PRELUDE.trim())?;
        writeln!(buffer, "{}", self.replace(example.trim()))?;
        writeln!(buffer, "```")?;
      }
    }

    if buffer.ends_with('\n') {
      let _: Option<char> = buffer.pop();
    }

    Ok(buffer)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo::rerun-if-changed=src/docs/sint");
  println!("cargo::rerun-if-changed=src/docs/uint");

  let root: OsString = env::var_os("OUT_DIR").unwrap();
  let root: PathBuf = Path::new(root.as_os_str()).join("docs");

  let uint_path: PathBuf = root.join("uint");
  let sint_path: PathBuf = root.join("int");

  fs::create_dir_all(uint_path.as_path())?;
  fs::create_dir_all(sint_path.as_path())?;

  for source in UINT_SOURCES {
    let doc_map: HashMap<&'static str, Docs<true>> = from_str(source)?;
    let output: &Path = uint_path.as_path();

    for (name, docs) in doc_map.iter() {
      let output: PathBuf = output.join(name).with_extension("md");
      let buffer: String = docs.to_docstring()?;
      fs::write(output, buffer)?;
    }
  }

  for source in SINT_SOURCES {
    let doc_map: HashMap<&'static str, Docs<false>> = from_str(source)?;
    let output: &Path = sint_path.as_path();

    for (name, docs) in doc_map.iter() {
      let output: PathBuf = output.join(name).with_extension("md");
      let buffer: String = docs.to_docstring()?;
      fs::write(output, buffer)?;
    }
  }

  Ok(())
}
