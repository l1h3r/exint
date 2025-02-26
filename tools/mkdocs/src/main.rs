//! Build exint documentation from TOML files.

use mkdocs::SourceList;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

const WRAPPERS: &[&str] = &["Saturating", "Strict", "Wrapping"];

fn main() -> Result<(), Error> {
  let source: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
  let parent: &Path = source.parent().and_then(Path::parent).unwrap();
  let target: PathBuf = parent.join("lib").join("exint-integer").join("src").join("docs");

  // Make sure this is a crate root
  assert!(parent.join("Cargo.toml").exists());

  SourceList::SINT_TY.build_base_ty(target.join("int"))?;
  SourceList::UINT_TY.build_base_ty(target.join("uint"))?;

  for wrapper in WRAPPERS {
    SourceList::WRAPPER.build_wrapper(target.join("int").join(wrapper), wrapper, "int")?;
    SourceList::WRAPPER.build_wrapper(target.join("uint").join(wrapper), wrapper, "uint")?;
  }

  Ok(())
}
