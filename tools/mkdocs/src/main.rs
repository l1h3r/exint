//! Build exint documentation from TOML files.

use mkdocs::DocMap;
use mkdocs::SourceList;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  let source: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));
  let parent: &Path = source.parent().and_then(Path::parent).unwrap();
  let target: PathBuf = parent.join("src").join("docs");

  // Make sure this is a crate root
  assert!(parent.join("Cargo.toml").exists());

  let paths: [PathBuf; 2] = [target.join("int"), target.join("uint")];

  fs::create_dir_all(paths[0].as_path())?;
  fs::create_dir_all(paths[1].as_path())?;

  for source in SourceList::iter() {
    let uint: bool = source.uint();
    let docs: DocMap = DocMap::parse(source)?;

    for (name, data) in docs.iter() {
      let output: &Path = paths[uint as usize].as_path();
      let output: PathBuf = output.join(name).with_extension("md");

      let writer: File = File::create(output)?;
      let writer: BufWriter<File> = BufWriter::new(writer);

      data.write_docstring(writer, uint)?;
    }
  }

  Ok(())
}
