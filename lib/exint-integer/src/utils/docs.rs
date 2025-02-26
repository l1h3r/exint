macro_rules! include_doc {
  ($type:ident, $name:literal) => {
    $crate::utils::include_doc!(concat!(stringify!($type), "/", $name))
  };
  ($outer:ident, $inner:ident, $name:literal) => {
    $crate::utils::include_doc!(concat!(stringify!($inner), "/", stringify!($outer), "/", $name))
  };
  ($file:expr_2021) => {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/", $file, ".md"))
  };
}

macro_rules! must_use_doc {
  () => {
    "this returns the result of the operation, without modifying the original"
  };
  ("to_ascii_uppercase") => {
    "to uppercase the value in-place, use `make_ascii_uppercase()`"
  };
  ("to_ascii_lowercase") => {
    "to lowercase the value in-place, use `make_ascii_lowercase()`"
  };
}

pub(crate) use include_doc;
pub(crate) use must_use_doc;
