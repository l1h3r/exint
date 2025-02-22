use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::instance::Instance;
use crate::function::Function;

pub(crate) struct Context {
  data: Vec<Instance>,
}

impl Context {
  pub(crate) fn new(capacity: usize) -> Self {
    Self {
      data: Vec::with_capacity(capacity),
    }
  }

  pub(crate) fn build<F>(&mut self, name: &'static str, f: F)
  where
    F: Fn(Function) -> Function,
  {
    self.data.push(Instance::new(name, f));
  }

  pub(crate) fn write(&self, root: &Path) -> Result<(), Error> {
    for instance in self.data.iter() {
      let path: PathBuf = root.join(instance.name).with_extension("rs");
      let file: File = File::create(path)?;

      BufWriter::new(file).write_fmt(format_args!("{instance}"))?;
    }

    Ok(())
  }
}
