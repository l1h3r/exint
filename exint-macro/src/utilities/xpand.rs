use core::iter::Peekable;
use core::ops::Deref;
use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use proc_macro::token_stream::IntoIter;

use crate::backports::ToTokens;
use crate::error::Error;
use crate::utilities::SpanExt;

pub(crate) struct Context<'a, T> {
  iter: &'a mut Peekable<IntoIter>,
  data: T,
}

impl<'a, T> Context<'a, T> {
  pub(crate) const fn new(iter: &'a mut Peekable<IntoIter>, data: T) -> Self {
    Self { iter, data }
  }

  pub(crate) fn split(self) -> (T, &'a mut Peekable<IntoIter>) {
    (self.data, self.iter)
  }
}

impl<T> Deref for Context<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

pub(crate) trait Expand {
  fn expand(&mut self, input: TokenStream) -> Result<TokenStream, Error> {
    let mut stream: TokenStream = TokenStream::new();

    self.token_stream(&mut stream, input)?;

    Ok(stream)
  }

  fn token_stream(&mut self, stream: &mut TokenStream, input: TokenStream) -> Result<(), Error> {
    let mut iter: Peekable<IntoIter> = input.into_iter().peekable();

    while let Some(token) = iter.next() {
      self.token_tree(stream, Context::new(&mut iter, token))?;
    }

    Ok(())
  }

  fn token_tree(
    &mut self,
    stream: &mut TokenStream,
    input: Context<'_, TokenTree>,
  ) -> Result<(), Error> {
    match input.data {
      TokenTree::Group(item) => self.group(stream, Context::new(input.iter, item))?,
      TokenTree::Ident(item) => self.ident(stream, Context::new(input.iter, item))?,
      TokenTree::Punct(item) => self.punct(stream, Context::new(input.iter, item))?,
      TokenTree::Literal(item) => self.value(stream, Context::new(input.iter, item))?,
    }

    Ok(())
  }

  fn group(&mut self, stream: &mut TokenStream, input: Context<'_, Group>) -> Result<(), Error> {
    if input.delimiter() == Delimiter::None {
      return self.token_stream(stream, input.stream());
    }

    let value: TokenStream = self.expand(input.stream())?;
    let group: Group = SpanExt::new_group(input.span(), value, input.delimiter());

    group.to_tokens(stream);

    Ok(())
  }

  fn ident(&mut self, stream: &mut TokenStream, input: Context<'_, Ident>) -> Result<(), Error> {
    input.to_tokens(stream);
    Ok(())
  }

  fn punct(&mut self, stream: &mut TokenStream, input: Context<'_, Punct>) -> Result<(), Error> {
    input.to_tokens(stream);
    Ok(())
  }

  fn value(&mut self, stream: &mut TokenStream, input: Context<'_, Literal>) -> Result<(), Error> {
    input.to_tokens(stream);
    Ok(())
  }
}
