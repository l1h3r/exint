macro_rules! include_doc {
  ($type:ident, $name:literal) => {
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/", stringify!($type), "/", $name, ".md"))
  };
}

macro_rules! must_use_doc {
  () => {
    "this returns the result of the operation, without modifying the original"
  };
}

pub(crate) use include_doc;
pub(crate) use must_use_doc;
