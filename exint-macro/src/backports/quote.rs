#[doc(hidden)]
macro_rules! __quote_internal_token {
  ($span:expr => ($($tt:tt)*)) => {
    $crate::utilities::SpanExt::new_parenthesised($span, quote_spanned!($span => $($tt)*))
  };
  ($span:expr => {$($tt:tt)*}) => {
    $crate::utilities::SpanExt::new_braced($span, quote_spanned!($span => $($tt)*))
  };
  ($span:expr => [$($tt:tt)*]) => {
    $crate::utilities::SpanExt::new_bracketed($span, quote_spanned!($span => $($tt)*))
  };

  ($span:expr => ,) => { $crate::utilities::SpanExt::new_punct_alone($span, ',') };
  ($span:expr => .) => { $crate::utilities::SpanExt::new_punct_alone($span, '.') };
  ($span:expr => ;) => { $crate::utilities::SpanExt::new_punct_alone($span, ';') };
  ($span:expr => !) => { $crate::utilities::SpanExt::new_punct_alone($span, '!') };
  ($span:expr => <) => { $crate::utilities::SpanExt::new_punct_alone($span, '<') };
  ($span:expr => >) => { $crate::utilities::SpanExt::new_punct_alone($span, '>') };
  ($span:expr => &) => { $crate::utilities::SpanExt::new_punct_alone($span, '&') };
  ($span:expr => =) => { $crate::utilities::SpanExt::new_punct_alone($span, '=') };
  ($span:expr => #) => { $crate::utilities::SpanExt::new_punct_alone($span, '#') };
  ($span:expr => |) => { $crate::utilities::SpanExt::new_punct_alone($span, '|') };
  ($span:expr => :) => { $crate::utilities::SpanExt::new_punct_alone($span, ':') };
  ($span:expr => *) => { $crate::utilities::SpanExt::new_punct_alone($span, '*') };

  ($span:expr => _) => {
    $crate::utilities::SpanExt::new_ident($span, "_")
  };
  ($span:expr => $value:ident) => {
    $crate::utilities::SpanExt::new_ident($span, stringify!($value))
  };
  ($_span:expr => $value:literal) => {
    stringify!($value).parse::<::proc_macro::Literal>().unwrap()
  };
}

#[doc(hidden)]
macro_rules! __quote_internal_stream {
  ($_span:expr => (@$($tt:tt)*)) => {
    $($tt)*
  };
  ($span:expr => (#($($tt:tt)*),*) ) => {{
    let mut stream: ::proc_macro::TokenStream = ::proc_macro::TokenStream::new();

    for (index, value) in ($($tt)*).iter().enumerate() {
      if index != 0 {
        $crate::backports::ToTokens::to_tokens(
          &$crate::utilities::SpanExt::new_punct_alone($span, ','),
          &mut stream,
        );
      }

      $crate::backports::ToTokens::to_tokens(value, &mut stream);
    }

    stream
  }};
  ($span:expr => ::) => {
    <::proc_macro::TokenStream as ::core::iter::FromIterator<_>>::from_iter([
      ::proc_macro::TokenTree::from($crate::utilities::SpanExt::new_punct_joint($span, ':')),
      ::proc_macro::TokenTree::from($crate::utilities::SpanExt::new_punct_alone($span, ':')),
    ])
  };
  ($span:expr => $tt:tt) => {
    ::proc_macro::TokenTree::from(__quote_internal_token!($span => $tt))
  };
}

macro_rules! quote_spanned {
  ($span:expr=> $($tt:tt)*) => {{
    #[allow(unused_mut)]
    let mut stream: ::proc_macro::TokenStream = ::proc_macro::TokenStream::new();

    $(
      $crate::backports::ToTokens::to_tokens(&__quote_internal_stream!($span => $tt), &mut stream);
    )*

    stream
  }};
}

#[expect(unused_macros)]
macro_rules! quote {
  ($($tt:tt)*) => {
    quote_spanned!(::proc_macro::Span::call_site() => $($tt)*)
  };
}
