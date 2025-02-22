use crate::condition::Condition;
use crate::type_name::TypeName;

pub(crate) struct Function {
  pub(crate) type_name: TypeName,
  pub(crate) signature: &'static str,
  pub(crate) statements: Vec<&'static str>,
  pub(crate) filechecks: Vec<(&'static str, Condition)>,
  pub(crate) skip_emit: Option<Condition>,
}

impl Function {
  pub(crate) const fn new(type_name: TypeName) -> Self {
    Self {
      type_name,
      signature: "",
      statements: Vec::new(),
      filechecks: Vec::new(),
      skip_emit: None,
    }
  }

  pub(crate) fn signature(mut self, value: &'static str) -> Self {
    self.signature = value;
    self
  }

  pub(crate) fn statement(mut self, value: &'static str) -> Self {
    self.statements.push(value);
    self
  }

  pub(crate) fn filecheck(mut self, value: &'static str, condition: Condition) -> Self {
    self.filechecks.push((value, condition));
    self
  }

  pub(crate) fn skip_emit(mut self, value: Condition) -> Self {
    self.skip_emit = Some(value);
    self
  }
}
