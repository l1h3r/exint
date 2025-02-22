use std::fs;
use std::io::Error;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
  let root: PathBuf = exint_compile::workspace_root()?;
  let test: PathBuf = root.join("tests/codegen");

  fs::create_dir_all(test.as_path())?;
  exint_filegen::build(test.as_path())?;

  Ok(())
}
