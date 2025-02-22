use basic_toml::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Error;

use crate::parse::DocStr;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct DocMap {
  #[serde(borrow)]
  data: HashMap<&'static str, DocStr>,
}

impl DocMap {
  pub fn parse(source: &'static str) -> Result<Self, Error> {
    from_str(source).map_err(Error::other)
  }

  pub fn iter(&self) -> impl Iterator<Item = (&'static str, &DocStr)> + '_ {
    self.data.iter().map(|(key, value)| (*key, value))
  }
}
