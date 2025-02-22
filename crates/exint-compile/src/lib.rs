//! Test runner for LLVM FileCheck
//!
//! https://llvm.org/docs/LangRef.html
//! https://llvm.org/docs/CommandGuide/FileCheck.html

mod capture;
mod command;
mod context;
mod support;

pub use self::support::workspace_root;

pub mod filecheck {
  use std::io::Error;
  use std::path::Path;

  use crate::command;
  use crate::command::BuildType;
  use crate::command::Config;
  use crate::command::Crate;
  use crate::context::Context;

  pub fn verify<P>(path: &P)
  where
    P: AsRef<Path> + ?Sized,
  {
    if let Err(error) = __verify(path) {
      panic!("{error}");
    }
  }

  fn __verify<P>(path: &P) -> Result<(), Error>
  where
    P: AsRef<Path> + ?Sized,
  {
    let context: Context = Context::new(path.as_ref())?;

    // TODO: Cache and avoid rebuild
    let backend: Crate = {
      let config: Config = Config {
        kind: BuildType::RLib,
        name: "exint_backend",
        path: context.crate_lib("exint-backend"),
        deps: Vec::new(),
      };

      command::rustc(&context, config)?
    };

    // TODO: Cache and avoid rebuild
    let integer: Crate = {
      let config: Config = Config {
        kind: BuildType::RLib,
        name: "exint_integer",
        path: context.crate_lib("exint-integer"),
        deps: vec![
          (backend.name, backend.output()),
        ],
      };

      command::rustc(&context, config)?
    };

    let codegen: Crate = {
      let config: Config = Config {
        kind: BuildType::LLVM,
        name: "exint_codegen",
        path: context.test_path.clone(),
        deps: vec![
          (backend.name, backend.output()),
          (integer.name, integer.output()),
        ],
      };

      command::rustc(&context, config)?
    };

    println!("[filecheck.verify.backend]: {:?}", backend.output());
    println!("[filecheck.verify.integer]: {:?}", integer.output());
    println!("[filecheck.verify.codegen]: {:?}", codegen.output());

    command::filecheck(&context, codegen.output())
  }
}
