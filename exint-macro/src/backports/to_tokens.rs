use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Literal;
use proc_macro::Punct;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::rc::Rc;

// -----------------------------------------------------------------------------
// ToTokens
// -----------------------------------------------------------------------------

pub(crate) trait ToTokens {
  fn to_tokens(&self, tokens: &mut TokenStream);

  fn to_token_stream(&self) -> TokenStream {
    let mut stream: TokenStream = TokenStream::new();
    self.to_tokens(&mut stream);
    stream
  }

  fn into_token_stream(self) -> TokenStream
  where
    Self: Sized,
  {
    self.to_token_stream()
  }
}

impl ToTokens for TokenStream {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend(self.clone());
  }

  fn into_token_stream(self) -> TokenStream {
    self
  }
}

impl ToTokens for TokenTree {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend(Some(self.clone()));
  }
}

impl ToTokens for Group {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    TokenTree::Group(self.clone()).to_tokens(tokens);
  }
}

impl ToTokens for Ident {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    TokenTree::Ident(self.clone()).to_tokens(tokens);
  }
}

impl ToTokens for Punct {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    TokenTree::Punct(self.clone()).to_tokens(tokens);
  }
}

impl ToTokens for Literal {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    TokenTree::Literal(self.clone()).to_tokens(tokens);
  }
}

impl<T: ToTokens + ?Sized> ToTokens for &T {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    (**self).to_tokens(tokens);
  }
}

impl<T: ToTokens + ?Sized> ToTokens for &mut T {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    (**self).to_tokens(tokens);
  }
}

impl<T: ToTokens + ?Sized> ToTokens for Box<T> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    (**self).to_tokens(tokens);
  }
}

impl<T: ToTokens + ?Sized> ToTokens for Rc<T> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    (**self).to_tokens(tokens);
  }
}

impl<T: ToTokens + ToOwned + ?Sized> ToTokens for Cow<'_, T> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    (**self).to_tokens(tokens);
  }
}

impl<T: ToTokens> ToTokens for Option<T> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if let Some(inner) = self {
      inner.to_tokens(tokens);
    }
  }
}

macro_rules! number {
  ($($type:ident => $func:ident),+ $(,)?) => {
    $(
      impl ToTokens for $type {
        fn to_tokens(&self, tokens: &mut TokenStream) {
          Literal::$func(*self).to_tokens(tokens);
        }
      }
    )+
  };
}

number! {
  u8 => u8_suffixed,
  u16 => u16_suffixed,
  u32 => u32_suffixed,
  u64 => u64_suffixed,
  u128 => u128_suffixed,
  usize => usize_suffixed,

  i8 => i8_suffixed,
  i16 => i16_suffixed,
  i32 => i32_suffixed,
  i64 => i64_suffixed,
  i128 => i128_suffixed,
  isize => isize_suffixed,

  f32 => f32_suffixed,
  f64 => f64_suffixed,
}

impl ToTokens for bool {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if *self {
      Ident::new("true", Span::call_site()).to_tokens(tokens);
    } else {
      Ident::new("false", Span::call_site()).to_tokens(tokens);
    }
  }
}

impl ToTokens for char {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Literal::character(*self).to_tokens(tokens);
  }
}

impl ToTokens for str {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Literal::string(self).to_tokens(tokens);
  }
}

impl ToTokens for String {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Literal::string(self).to_tokens(tokens);
  }
}

impl ToTokens for CStr {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Literal::c_string(self).to_tokens(tokens);
  }
}

impl ToTokens for CString {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    Literal::c_string(self).to_tokens(tokens);
  }
}
