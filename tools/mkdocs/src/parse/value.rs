use serde::Deserialize;
use std::borrow::Cow;
use std::iter;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Value {
  #[serde(borrow)]
  Item(Cow<'static, str>),
  #[serde(borrow)]
  List(Vec<Cow<'static, str>>),
}

impl Value {
  pub fn iter(&self) -> Box<dyn Iterator<Item = &str> + '_> {
    match self {
      Self::Item(item) => Box::new(iter::once(item.as_ref())),
      Self::List(list) => Box::new(list.iter().map(AsRef::as_ref)),
    }
  }
}
