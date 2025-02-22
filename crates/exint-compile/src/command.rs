use std::fs;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::capture::Capture;
use crate::context::Context;

pub(crate) struct Config {
  pub(crate) kind: BuildType,
  pub(crate) name: &'static str,
  pub(crate) path: PathBuf,
  pub(crate) deps: Vec<(&'static str, PathBuf)>,
}

// [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
#[derive(Debug)]
pub(crate) enum BuildType {
  RLib, // metadata,link
  LLVM, // llvm-ir
}

impl BuildType {
  const fn arg(&self) -> &'static str {
    match self {
      Self::RLib => "metadata,link",
      Self::LLVM => "llvm-ir",
    }
  }
}

pub(crate) struct Crate {
  pub(crate) kind: BuildType,
  pub(crate) name: &'static str,
  path_src: PathBuf,
  path_dst: PathBuf,
}

impl Crate {
  pub(crate) fn output(&self) -> PathBuf {
    match self.kind {
      BuildType::RLib => self.rmeta(),
      BuildType::LLVM => self.llvm_ir(),
    }
  }

  fn rmeta(&self) -> PathBuf {
    self
      .path_dst
      .join(format!("lib{}", self.name))
      .with_extension("rmeta")
  }

  fn llvm_ir(&self) -> PathBuf {
    self
      .path_dst
      .join(self.name)
      .with_extension("ll")
  }
}

pub(crate) fn filecheck(context: &Context, input: PathBuf) -> Result<(), Error> {
  // ---------------------------------------------------------------------------
  // 1. Build FileCheck command
  // ---------------------------------------------------------------------------

  let mut command: Command = Command::new(context.check.as_ref());

  command.arg("--input-file");
  command.arg(input);
  command.arg("--allow-unused-prefixes");
  command.arg(context.test_path.as_path());

  println!("[check.command]: {:?}", command);

  // ---------------------------------------------------------------------------
  // 2. Execute command; capture output
  // ---------------------------------------------------------------------------

  let capture: Capture = Capture::new(command)?;

  println!("[check.status]: {:?}", capture.status);
  println!("[check.stdout]: {:?}", capture.stdout);
  println!("[check.stderr]: {:?}", capture.stderr);

  capture.check()?;
  capture.write(context.artifacts.as_path())?;

  Ok(())
}

pub(crate) fn rustc(context: &Context, config: Config) -> Result<Crate, Error> {
  println!("[rustc.name]: {:?}", config.name);
  println!("[rustc.path]: {:?}", config.path);
  println!("[rustc.deps]: {:?}", config.deps);
  println!("[rustc.kind]: {:?}", config.kind);

  // ---------------------------------------------------------------------------
  // 1. Prepare output directory
  // ---------------------------------------------------------------------------

  fs::create_dir_all(context.deps_path.as_path())?;

  // ---------------------------------------------------------------------------
  // 2. Build rustc command
  // ---------------------------------------------------------------------------

  let mut command: Command = Command::new(context.rustc.as_ref());

  command.arg(config.path.as_path());
  command.arg("--verbose");

  command.arg("--edition");
  command.arg("2024"); // TODO: Make configurable

  command.arg("--crate-type");
  command.arg("lib"); // TODO: Make configurable

  command.arg("--crate-name");
  command.arg(config.name);

  command.arg("--emit");
  command.arg(config.kind.arg());

  command.arg("--out-dir");
  command.arg(format!("{}", context.deps_path.display()));

  command.arg("-L");
  command.arg(format!("dependency={}", context.deps_path.display()));

  for (name, path) in config.deps.iter() {
    command.arg("--extern");
    command.arg(format!("{}={}", name, path.display()));
  }

  command.arg("--codegen");
  command.arg("opt-level=3"); // TODO: Make configurable

  command.arg("--codegen");
  command.arg("embed-bitcode=no"); // TODO: Make configurable

  command.arg("--codegen");
  command.arg("strip=debuginfo"); // TODO: Make configurable

  // TODO: Get this info from .cargo/config.toml
  command.arg("--codegen");
  command.arg("llvm-args=--unroll-threshold=1024");

  println!("[rustc.command]: {:?}", command);

  // ---------------------------------------------------------------------------
  // 3. Execute command; capture output
  // ---------------------------------------------------------------------------

  let capture: Capture = Capture::new(command)?;

  println!("[rustc.status]: {:?}", capture.status);
  println!("[rustc.stdout]: {:?}", capture.stdout);
  println!("[rustc.stderr]: {:?}", capture.stderr);

  capture.check()?;
  capture.write(context.artifacts.as_path())?;

  // ---------------------------------------------------------------------------
  // 4. Build and return target identifier
  // ---------------------------------------------------------------------------

  Ok(Crate {
    kind: config.kind,
    name: config.name,
    path_src: config.path,
    path_dst: context.deps_path.clone(),
  })
}
