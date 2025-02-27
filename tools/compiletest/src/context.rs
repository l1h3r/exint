use std::borrow::Cow;
use std::env;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use crate::command::BuildType;
use crate::command::Config;
use crate::command::Crate;
use crate::support;
use crate::support::meta_path;

pub(crate) struct Context {
  pub(crate) rustc: Cow<'static, str>,
  pub(crate) check: Cow<'static, str>,
  pub(crate) root_path: PathBuf,
  pub(crate) test_path: PathBuf,
  pub(crate) artifacts: PathBuf,
  pub(crate) deps_path: PathBuf,
}

impl Context {
  pub(crate) fn new(path: &Path) -> Result<Self, Error> {
    let root_path: PathBuf = support::workspace_root()?;
    let test_path: PathBuf = root_path.join("tests").join(path);
    let artifacts: PathBuf = root_path.join("tests").join(".artifacts");
    let deps_path: PathBuf = artifacts.join("deps");

    let rustc: Cow<'static, str> = try_env("EXINT_RUSTC", "rustc");
    let check: Cow<'static, str> = try_env("EXINT_FILECHECK", "FileCheck");

    Ok(Self {
      rustc,
      check,
      root_path,
      test_path,
      artifacts,
      deps_path,
    })
  }

  pub(crate) fn crate_root(&self, name: &str) -> PathBuf {
    self.root_path.join("lib").join(name)
  }

  pub(crate) fn crate_lib(&self, name: &str) -> PathBuf {
    self.crate_root(name).join("src").join("lib.rs")
  }

  pub(crate) fn find_dep(&self, config: &Config) -> Option<Crate> {
    let path: PathBuf = meta_path(self.deps_path.as_path(), config.name);

    if !path.exists() {
      return None;
    }

    let krate: Crate = Crate {
      kind: BuildType::RLib,
      name: config.name,
      path_src: config.path.clone(),
      path_dst: self.deps_path.clone(),
    };

    Some(krate)
  }
}

fn try_env(key: &str, default: &'static str) -> Cow<'static, str> {
  if let Ok(value) = env::var(key) {
    let value: &str = value.trim();

    if !value.is_empty() {
      return Cow::Owned(value.to_owned());
    }
  }

  Cow::Borrowed(default)
}
