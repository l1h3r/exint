//! Test runner for LLVM FileCheck
//!
//! https://llvm.org/docs/LangRef.html
//! https://llvm.org/docs/CommandGuide/FileCheck.html

#![allow(dead_code)]

mod capture;
mod command;
mod context;
mod support;

pub mod filecheck {
  use std::fs;
  use std::fs::DirEntry;
  use std::io::Error;
  use std::path::Path;
  use std::path::PathBuf;

  use crate::command;
  use crate::command::BuildType;
  use crate::command::Config;
  use crate::command::Crate;
  use crate::context::Context;
  use crate::support;

  const EXINT_BACKEND_FEATS: &[&'static str] = &[
    "bigint_helper_methods",
    "const_trait_impl",
    "core_intrinsics",
    "disjoint_bitor",
    "internal_features",
    "min_specialization",
    "nightly",
  ];

  const EXINT_INTEGER_FEATS: &[&'static str] = &[
    "adt_const_params",
    "ascii_char",
    "bigint_helper_methods",
    "const_trait_impl",
    "core_intrinsics",
    "disjoint_bitor",
    "f128",
    "f16",
    "incomplete_features",
    "int_from_ascii",
    "int_roundings",
    "integer_atomics",
    "internal_features",
    "is_ascii_octdigit",
    "isolate_most_least_significant_one",
    "min_specialization",
    "mixed_integer_ops_unsigned_sub",
    "never_type",
    "nightly",
    "random",
    "std",
    "step_trait",
    "strict_overflow_ops",
    "structural_match",
    "trusted_step",
    "unchecked_neg",
    "unchecked_shifts",
    "unsigned_signed_diff",
    "unsized_const_params",
    "utf16_extra",
    "wrapping_next_power_of_two",
  ];

  pub fn purge() {
    if let Err(error) = __purge() {
      panic!("{error}");
    }
  }

  fn __purge() -> Result<(), Error> {
    let root_path: PathBuf = support::workspace_root()?;
    let artifacts: PathBuf = root_path.join("tests").join(".artifacts");

    if !artifacts.exists() {
      return Ok(());
    }

    for entry in fs::read_dir(artifacts)? {
      let data: DirEntry = entry?;
      let path: PathBuf = data.path();

      if path.is_dir() {
        fs::remove_dir_all(path)?;
      } else if path.is_file() {
        fs::remove_file(path)?;
      }
    }

    Ok(())
  }

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

    let backend: Crate = {
      let config: Config = Config {
        kind: BuildType::RLib,
        name: "exint_backend",
        path: context.crate_lib("exint-backend"),
        root: Some(context.crate_root("exint-backend")),
        deps: Vec::new(),
        features: EXINT_BACKEND_FEATS,
      };

      if let Some(krate) = context.find_dep(&config) {
        krate
      } else {
        command::rustc(&context, config)?
      }
    };

    let integer: Crate = {
      let config: Config = Config {
        kind: BuildType::RLib,
        name: "exint_integer",
        path: context.crate_lib("exint-integer"),
        root: Some(context.crate_root("exint-integer")),
        deps: vec![
          (backend.name, backend.output()),
        ],
        features: EXINT_INTEGER_FEATS,
      };

      if let Some(krate) = context.find_dep(&config) {
        krate
      } else {
        command::rustc(&context, config)?
      }
    };

    let rootlib: Crate = {
      let config: Config = Config {
        kind: BuildType::RLib,
        name: "exint",
        path: context.root_path.join("src").join("lib.rs"),
        root: Some(context.root_path.clone()),
        deps: vec![
          (backend.name, backend.output()),
          (integer.name, integer.output()),
        ],
        features: &[],
      };

      if let Some(krate) = context.find_dep(&config) {
        krate
      } else {
        command::rustc(&context, config)?
      }
    };

    let codegen: Crate = {
      let config: Config = Config {
        kind: BuildType::LLVM,
        name: "exint_codegen",
        path: context.test_path.clone(),
        root: None,
        deps: vec![
          (rootlib.name, rootlib.output()),
        ],
        features: &[],
      };

      command::rustc(&context, config)?
    };

    println!("[filecheck.verify.backend]: {:?}", backend.output());
    println!("[filecheck.verify.integer]: {:?}", integer.output());
    println!("[filecheck.verify.rootlib]: {:?}", rootlib.output());
    println!("[filecheck.verify.codegen]: {:?}", codegen.output());

    command::filecheck(&context, codegen.output())
  }
}
