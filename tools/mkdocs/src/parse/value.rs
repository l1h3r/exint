use serde::Deserialize;
use std::iter;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Value {
  Item(&'static str),
  List(Vec<&'static str>),
}

impl Value {
  pub fn iter(&self) -> Box<dyn Iterator<Item = &'static str> + '_> {
    match self {
      Self::Item(item) => Box::new(iter::once(*item)),
      Self::List(list) => Box::new(list.iter().copied()),
    }
  }
}
