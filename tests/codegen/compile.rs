//! Test runner for LLVM FileCheck
//!
//! https://llvm.org/docs/LangRef.html
//! https://llvm.org/docs/CommandGuide/FileCheck.html

use rustc_driver::Callbacks;
use rustc_driver::catch_with_exit_code;
use rustc_driver::run_compiler;
use rustc_driver::EXIT_FAILURE;
use rustc_interface::Config;
use rustc_lint_defs::Level;
use rustc_session::config::CrateType;
use rustc_session::config::ExternEntry;
use rustc_session::config::ExternLocation;
use rustc_session::config::Externs;
use rustc_session::config::Input;
use rustc_session::config::OptLevel;
use rustc_session::config::OutputType;
use rustc_session::config::OutputTypes;
use rustc_session::search_paths::PathKind;
use rustc_session::search_paths::SearchPath;
use rustc_session::utils::CanonicalizedPath;
use rustc_span::edition::Edition;
use rustc_target::spec::MergeFunctions;
use rustc_target::spec::TargetTuple;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::sync::LazyLock;

// -----------------------------------------------------------------------------
// Context
// -----------------------------------------------------------------------------

pub(crate) struct Context {
  pub(crate) llvm: Cow<'static, str>,
  pub(crate) root: Cow<'static, Path>,
}

impl Context {
  #[inline]
  pub(crate) fn new() -> Result<Self, Error> {
    let this: Self = Self {
      llvm: Cow::Borrowed("FileCheck"),
      root: workspace_root().map(Cow::Owned)?,
    };

    fs::create_dir_all(this.deps_path())?;

    Ok(this)
  }

  pub(crate) fn root_path(&self) -> &Path {
    self.root.as_ref()
  }

  pub(crate) fn test_path(&self) -> PathBuf {
    self.root.join("tests")
  }

  pub(crate) fn deps_path(&self) -> PathBuf {
    self.artifacts().join("deps")
  }

  pub(crate) fn artifacts(&self) -> PathBuf {
    self.test_path().join(".artifacts")
  }

  #[inline]
  pub(crate) fn llvm(mut self, value: impl Into<Cow<'static, str>>) -> Self {
    self.llvm = value.into();
    self
  }
}

// -----------------------------------------------------------------------------
// Capture
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct Capture {
  pub(crate) kind: CrateType,
  pub(crate) name: &'static str,
  pub(crate) path: PathBuf,
}

impl Capture {
  pub(crate) const fn new_rlib(name: &'static str, path: PathBuf) -> Self {
    Self::with_type(name, path, CrateType::Rlib)
  }

  pub(crate) const fn new_proc_macro(name: &'static str, path: PathBuf) -> Self {
    Self::with_type(name, path, CrateType::ProcMacro)
  }

  pub(crate) const fn with_type(name: &'static str, path: PathBuf, kind: CrateType) -> Self {
    Self { name, path, kind }
  }

  pub(crate) fn check(self, context: &Context) -> Result<(), Error> {
    let output: Output = Command::new(context.llvm.as_ref())
      .arg("--color")
      .arg("--input-file")
      .arg(context.deps_path().join(format!("{}.ll", self.name)))
      .arg(self.path.as_path())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .stdin(Stdio::null())
      .spawn()?
      .wait_with_output()?;

    let stdout: Cow<'_, str> = String::from_utf8_lossy(output.stdout.as_slice());
    let stderr: Cow<'_, str> = String::from_utf8_lossy(output.stderr.as_slice());

    fs::write(context.artifacts().join("out"), stdout.as_ref())?;
    fs::write(context.artifacts().join("err"), stderr.as_ref())?;

    if !output.status.success() {
      return Err(Error::other(stderr));
    }

    Ok(())
  }

  pub(crate) fn rustc(callbacks: &mut (dyn Callbacks + Send)) -> Result<(), Error> {
    let code: i32 = catch_with_exit_code(|| {
      run_compiler(&[const { String::new() }; 2], callbacks)
    });

    if code == EXIT_FAILURE {
      Err(Error::other("RUSTC"))
    } else {
      Ok(())
    }
  }

  pub(crate) fn to_extern_entry(&self, deps: &Path) -> ExternEntry {
    let path: CanonicalizedPath = self.extern_path(deps);

    ExternEntry {
      location: ExternLocation::ExactPaths(BTreeSet::from([path])),
      is_private_dep: false,
      add_prelude: true,
      nounused_dep: false,
      force: false,
    }
  }

  fn extern_path(&self, deps: &Path) -> CanonicalizedPath {
    match self.kind {
      CrateType::Rlib => {
        CanonicalizedPath::new(deps.join(format!("lib{}.rlib", self.name)))
      }
      CrateType::Cdylib | CrateType::Dylib | CrateType::ProcMacro | CrateType::Sdylib => {
        CanonicalizedPath::new(deps.join(format!("lib{}.dylib", self.name)))
      }
      CrateType::Executable => {
        panic!("not supported: Executable")
      }
      CrateType::Staticlib => {
        panic!("not supported: Staticlib")
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Callbacks
// -----------------------------------------------------------------------------

const PROC_MACRO_EXTERN: ExternEntry = ExternEntry {
  location: ExternLocation::FoundInLibrarySearchDirectories,
  is_private_dep: false,
  add_prelude: true,
  nounused_dep: false,
  force: false,
};

static LINT_OPTS: LazyLock<Vec<(String, Level)>> = LazyLock::new(|| {
  vec![
    ("incomplete_features".to_owned(), Level::Allow),
    ("internal_features".to_owned(), Level::Allow),
  ]
});

static LLVM_ARGS: LazyLock<Vec<String>> = LazyLock::new(|| {
  vec![
    "--aarch64-neon-syntax=apple".to_owned(),
    "--x86-asm-syntax=intel".to_owned(),
    "--unroll-threshold=9999".to_owned(),
  ]
});

// -----------------------------------------------------------------------------
// Callbacks (Default)
// -----------------------------------------------------------------------------

pub(crate) struct CallbacksDefault {
  externs: BTreeMap<String, ExternEntry>,
  features: BTreeSet<&'static str>,
  root_path: PathBuf,
  deps_path: PathBuf,
  crate_name: &'static str,
}

impl CallbacksDefault {
  const TARGET: &'static str = "aarch64-apple-darwin";

  pub(crate) fn new(name: &'static str, root: &Path, deps: &Path) -> Self {
    Self {
      externs: BTreeMap::new(),
      features: BTreeSet::new(),
      root_path: root.to_path_buf(),
      deps_path: deps.to_path_buf(),
      crate_name: name,
    }
  }

  pub(crate) fn features<I>(&mut self, name: I)
  where
    I: IntoIterator<Item = &'static str>,
  {
    self.features.extend(name);
  }

  pub(crate) fn dependency(&mut self, deps: &Path, krate: &Capture) {
    let _unused: Option<ExternEntry> = self
      .externs
      .insert(krate.name.to_owned(), krate.to_extern_entry(deps));
  }

  fn crate_cfg(&self) -> Vec<String> {
    fn __transform(name: &'static str) -> String {
      let mut buffer: String = String::with_capacity(10 + name.len());
      buffer.push_str("feature=");
      buffer.push('"');
      buffer.push_str(name);
      buffer.push('"');
      buffer
    }

    self.features.iter().copied().map(__transform).collect()
  }
}

impl Callbacks for CallbacksDefault {
  fn config(&mut self, config: &mut Config) {
    // -------------------------------------------------------------------------
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_interface/interface/struct.Config.html
    // -------------------------------------------------------------------------

    config.crate_cfg = self.crate_cfg();
    config.input = Input::File(self.root_path.clone());
    config.output_dir = Some(self.deps_path.clone());

    // -------------------------------------------------------------------------
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/options/struct.Options.html
    // -------------------------------------------------------------------------

    config.opts.crate_name = Some(self.crate_name.to_owned());
    config.opts.debug_assertions = false;
    config.opts.edition = Edition::Edition2024;
    config.opts.externs = Externs::new(self.externs.clone());
    config.opts.lint_opts = LINT_OPTS.clone();
    config.opts.optimize = OptLevel::Aggressive;
    config.opts.search_paths = [SearchPath::new(PathKind::Dependency, self.deps_path.clone())].to_vec();
    config.opts.target_triple = TargetTuple::TargetTuple(Self::TARGET.to_owned());
    config.opts.verbose = true;

    // -------------------------------------------------------------------------
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/config/struct.CodegenOptions.html
    // -------------------------------------------------------------------------

    config.opts.cg.debug_assertions = Some(config.opts.debug_assertions);
    config.opts.cg.llvm_args = LLVM_ARGS.clone();
    config.opts.cg.opt_level = "3".to_owned();

    // -------------------------------------------------------------------------
    // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/config/struct.UnstableOptions.html
    // -------------------------------------------------------------------------

    // Ensure consistent order of compiled output
    config.opts.unstable_opts.codegen_source_order = true;

    // Disable function merging - messes with LLVM FileCheck
    config.opts.unstable_opts.merge_functions = Some(MergeFunctions::Disabled);
  }
}

// -----------------------------------------------------------------------------
// Callbacks (RLib)
// -----------------------------------------------------------------------------

pub(crate) struct CallbacksRLib {
  default: CallbacksDefault,
}

impl CallbacksRLib {
  pub(crate) const fn new(default: CallbacksDefault) -> Self {
    Self { default }
  }
}

impl Callbacks for CallbacksRLib {
  fn config(&mut self, config: &mut Config) {
    self.default.config(config);

    config.opts.crate_types = [CrateType::Rlib].to_vec();

    config.opts.output_types = OutputTypes::new(&[
      (OutputType::DepInfo, None),
      (OutputType::Exe, None),
      (OutputType::Metadata, None),
    ]);
  }
}

// -----------------------------------------------------------------------------
// Callbacks (Assembly)
// -----------------------------------------------------------------------------

pub(crate) struct CallbacksAssembly {
  default: CallbacksDefault,
}

impl CallbacksAssembly {
  pub(crate) const fn new(default: CallbacksDefault) -> Self {
    Self { default }
  }
}

impl Callbacks for CallbacksAssembly {
  fn config(&mut self, config: &mut Config) {
    self.default.config(config);

    config.opts.crate_types = [CrateType::Rlib].to_vec();

    config.opts.output_types = OutputTypes::new(&[
      (OutputType::Assembly, None),
      (OutputType::LlvmAssembly, None),
    ]);
  }
}

// -----------------------------------------------------------------------------
// Callbacks (Proc Macro)
// -----------------------------------------------------------------------------

pub(crate) struct CallbacksProcMacro {
  default: CallbacksDefault,
}

impl CallbacksProcMacro {
  pub(crate) const fn new(default: CallbacksDefault) -> Self {
    Self { default }
  }
}

impl Callbacks for CallbacksProcMacro {
  fn config(&mut self, config: &mut Config) {
    let _unused: Option<ExternEntry> = self
      .default
      .externs
      .insert("proc_macro".to_owned(), PROC_MACRO_EXTERN);

    self.default.config(config);

    config.opts.crate_types = [CrateType::ProcMacro].to_vec();

    config.opts.output_types = OutputTypes::new(&[
      (OutputType::DepInfo, None),
      (OutputType::Exe, None),
    ]);
  }
}

// -----------------------------------------------------------------------------
// Misc. Utilities
// -----------------------------------------------------------------------------

fn locate_project() -> Result<Output, Error> {
  let output: Output = Command::new("cargo")
    .arg("locate-project")
    .arg("--workspace")
    .arg("--message-format")
    .arg("plain")
    .output()?;

  if output.status.success() {
    return Ok(output);
  }

  let content: &[u8] = output.stderr.as_slice();
  let message: Cow<'_, str> = String::from_utf8_lossy(content);

  Err(Error::other(message))
}

fn workspace_root() -> Result<PathBuf, Error> {
  let output: Output = locate_project()?;
  let stdout: &[u8] = output.stdout.as_slice();

  str::from_utf8(stdout)
    .ok()
    .map(str::trim)
    .map(Path::new)
    .and_then(Path::parent)
    .map(Path::to_path_buf)
    .ok_or_else(|| Error::other("failed to read path from `cargo locate-project`"))
}
