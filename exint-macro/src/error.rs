use proc_macro::Span;
use proc_macro::TokenStream;
use std::fmt::Display;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct Error {
  span: Span,
  data: String,
}

impl Error {
  #[inline]
  pub(crate) fn new<T>(data: T, span: Span) -> Self
  where
    T: Display,
  {
    Self {
      span,
      data: data.to_string(),
    }
  }

  #[inline]
  pub(crate) fn into_compile_error(self) -> TokenStream {
    quote_spanned!(self.span => ::core::compile_error!((@self.data)))
  }
}
