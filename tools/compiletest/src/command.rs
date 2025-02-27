use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

use crate::capture::Capture;
use crate::context::Context;
use crate::support::llvm_path;
use crate::support::meta_path;

pub(crate) struct Config<'a> {
  pub(crate) kind: BuildType,
  pub(crate) name: &'static str,
  pub(crate) path: PathBuf,
  pub(crate) root: Option<PathBuf>,
  pub(crate) deps: Vec<(&'a str, PathBuf)>,
  pub(crate) features: &'static [&'static str],
}

// [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]
#[derive(Debug)]
pub(crate) enum BuildType {
  RLib,
  LLVM,
}

impl BuildType {
  const fn arg(&self) -> &'static str {
    match self {
      Self::RLib => "dep-info,metadata,link",
      Self::LLVM => "llvm-ir,asm",
    }
  }
}

pub(crate) struct Crate {
  pub(crate) kind: BuildType,
  pub(crate) name: &'static str,
  pub(crate) path_src: PathBuf,
  pub(crate) path_dst: PathBuf,
}

impl Crate {
  pub(crate) fn output(&self) -> PathBuf {
    match self.kind {
      BuildType::RLib => meta_path(self.path_dst.as_path(), self.name),
      BuildType::LLVM => llvm_path(self.path_dst.as_path(), self.name),
    }
  }
}

pub(crate) fn filecheck(context: &Context, input: PathBuf) -> Result<(), Error> {
  // ---------------------------------------------------------------------------
  // 1. Build FileCheck command
  // ---------------------------------------------------------------------------

  let mut command: Command = Command::new(context.check.as_ref());

  command.arg("--input-file");
  command.arg(input);
  command.args(&["--dump-input-context", "100"]);
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
  println!("[rustc.kind]: {:?}", config.kind);
  println!("[rustc.name]: {:?}", config.name);
  println!("[rustc.path]: {:?}", config.path);
  println!("[rustc.root]: {:?}", config.root);
  println!("[rustc.deps]: {:?}", config.deps);

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

  command.arg("-Z");
  command.arg("merge-functions=disabled");

  for feature in config.features {
    command.arg("--cfg");
    command.arg(format!("feature=\"{feature}\""));
  }

  // ---------------------------------------------------------------------------
  // 3. Set mock Cargo environment
  // ---------------------------------------------------------------------------

  if matches!(config.kind, BuildType::RLib) {
    let Some(root) = config.root else {
      return Err(Error::other("Crate configuration is missing `config.root`."));
    };

    command.env("CARGO_CRATE_NAME", config.name.replace('-', "_"));
    command.env("CARGO_MANIFEST_DIR", root.as_path());
    command.env("CARGO_MANIFEST_PATH", root.join("Cargo.toml"));
  }

  // ---------------------------------------------------------------------------
  // 4. Execute command; capture output
  // ---------------------------------------------------------------------------

  println!("[rustc.command]: {:?}", command);

  let capture: Capture = Capture::new(command)?;

  println!("[rustc.status]: {:?}", capture.status);
  println!("[rustc.stdout]: {:?}", capture.stdout);
  println!("[rustc.stderr]: {:?}", capture.stderr);

  capture.check()?;
  capture.write(context.artifacts.as_path())?;

  // ---------------------------------------------------------------------------
  // 5. Build and return target identifier
  // ---------------------------------------------------------------------------

  Ok(Crate {
    kind: config.kind,
    name: config.name,
    path_src: config.path,
    path_dst: context.deps_path.clone(),
  })
}
