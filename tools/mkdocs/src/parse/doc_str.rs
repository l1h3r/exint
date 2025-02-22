use serde::Deserialize;
use std::borrow::Cow;

use crate::parse::Value;

#[derive(Deserialize)]
pub struct DocStr {
  #[serde(borrow)]
  overview: Cow<'static, str>,
  examples: Option<Value>,
  examples_overflow: Option<Value>,
  examples_div_zero: Option<Value>,
  examples_panicking: Option<Value>,
}

impl DocStr {
  pub fn overview(&self) -> &str {
    self.overview.as_ref()
  }

  pub const fn examples(&self) -> Option<&Value> {
    self.examples.as_ref()
  }

  pub const fn examples_overflow(&self) -> Option<&Value> {
    self.examples_overflow.as_ref()
  }

  pub const fn examples_div_zero(&self) -> Option<&Value> {
    self.examples_div_zero.as_ref()
  }

  pub const fn examples_panicking(&self) -> Option<&Value> {
    self.examples_panicking.as_ref()
  }

  pub const fn has_examples(&self) -> bool {
    self.examples.is_some()
      || self.examples_overflow.is_some()
      || self.examples_div_zero.is_some()
      || self.examples_panicking.is_some()
  }
}
