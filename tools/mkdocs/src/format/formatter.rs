use aho_corasick::AhoCorasick;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::sync::LazyLock;

use crate::format::KEYS;
use crate::format::VARS;

static AHO: LazyLock<AhoCorasick> = LazyLock::new(|| AhoCorasick::new(KEYS).unwrap());

pub struct StringFmt<'a> {
  value: &'a str,
  extra_keys: AhoCorasick,
  extra_vars: &'a [&'static str],
}

impl<'a> StringFmt<'a> {
  pub fn new(value: &'a str, extra_keys: &[&'static str], extra_vars: &'a [&'static str]) -> Self {
    assert!(
      extra_keys.len() == extra_vars.len(),
      "format requires a replacement for every pattern"
    );

    Self {
      value,
      extra_keys: AhoCorasick::new(extra_keys).unwrap(),
      extra_vars,
    }
  }

  pub fn set_value(&mut self, value: &'a str) {
    self.value = value;
  }

  fn format(&self) -> String {
    let string: &str = self.value.trim();
    let output: String = AHO.replace_all(string, VARS);

    if self.extra_vars.is_empty() {
      return output;
    }

    self
      .extra_keys
      .replace_all(output.as_str(), self.extra_vars)
  }
}

impl Display for StringFmt<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_str(self.format().as_str())
  }
}
