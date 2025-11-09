use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;

pub(crate) trait SpanExt: Sized {
  fn eq(&self, other: &Self) -> bool;
  fn at(&self) -> usize;

  fn new_group(self, value: TokenStream, delimiter: Delimiter) -> Group;
  fn new_ident(self, value: &str) -> Ident;
  fn new_punct(self, value: char, spacing: Spacing) -> Punct;

  fn new_parenthesised(self, value: TokenStream) -> Group {
    self.new_group(value, Delimiter::Parenthesis)
  }

  fn new_braced(self, value: TokenStream) -> Group {
    self.new_group(value, Delimiter::Brace)
  }

  fn new_bracketed(self, value: TokenStream) -> Group {
    self.new_group(value, Delimiter::Bracket)
  }

  fn new_punct_joint(self, value: char) -> Punct {
    self.new_punct(value, Spacing::Joint)
  }

  fn new_punct_alone(self, value: char) -> Punct {
    self.new_punct(value, Spacing::Alone)
  }
}

impl SpanExt for Span {
  fn eq(&self, other: &Self) -> bool {
    self.at() == other.at()
  }

  fn at(&self) -> usize {
    self.line().wrapping_add(self.column())
  }

  fn new_group(self, value: TokenStream, delimiter: Delimiter) -> Group {
    let mut this: Group = Group::new(delimiter, value);
    this.set_span(self);
    this
  }

  fn new_ident(self, value: &str) -> Ident {
    Ident::new(value, self)
  }

  fn new_punct(self, value: char, spacing: Spacing) -> Punct {
    let mut this: Punct = Punct::new(value, spacing);
    this.set_span(self);
    this
  }
}
