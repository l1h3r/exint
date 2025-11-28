use std::borrow::Cow;
use std::env;
use std::env::VarError;
use std::io::Error;
use std::path::PathBuf;
use std::sync::LazyLock;
use test::ShouldPanic;
use test::StaticTestName;
use test::TestDesc;
use test::TestType;

use super::compile::CallbacksAssembly;
use super::compile::CallbacksDefault;
use super::compile::CallbacksProcMacro;
use super::compile::CallbacksRLib;
use super::compile::Capture;
use super::compile::Context;

const FEATURES: &[&str] = &[
  "__internal_force_overflow_checks",
  "__internal_panic_immediate_abort",
  "std",
  // "all_const",
  "const_traits",
  "const_clone",
  "const_cmp",
  "const_convert",
  "const_default",
  "const_ops",
  "const_option",
  "const_result",
  // "all_backend",
  "core_intrinsics",
  "const_eval_select",
  "min_specialization",
  "portable_simd",
  // "all_unstable",
  "exact_bitshifts",
  "int_from_ascii",
  "int_lowest_highest_one",
  "int_roundings",
  "is_ascii_octdigit",
  "isolate_most_least_significant_one",
  "uint_bit_width",
  "utf16_extra",
  "wrapping_next_power_of_two",
  // "all_nightly",
  "adt_const_params",
  "ascii_char",
  "bigint_helper_methods",
  "disjoint_bitor",
  "exact_div",
  "f16",
  "f128",
  "funnel_shifts",
  "integer_atomics",
  "never_type",
  "random",
  "step_trait",
  "structural_match",
  "trusted_step",
  "unsized_const_params",
];

static FILECHECK: LazyLock<Option<Cow<'static, str>>> = LazyLock::new(|| {
  match env::var("LLVM_FILECHECK") {
    Ok(value) => Some(Cow::Owned(value)),
    Err(VarError::NotPresent) => None,
    Err(error @ VarError::NotUnicode(_)) => panic!("{error}"),
  }
});

static CONTEXT: LazyLock<Context> = LazyLock::new(|| {
  Context::new()
    .expect("context")
    .llvm(FILECHECK.clone().unwrap())
});

static EXINT_MACRO: LazyLock<Capture> = LazyLock::new(|| {
  let root: PathBuf = CONTEXT.root_path().join("exint-macro/src/lib.rs");
  let deps: PathBuf = CONTEXT.deps_path();

  let callbacks: CallbacksDefault = CallbacksDefault::new("exint_macro", &root, &deps);

  Capture::rustc(&mut CallbacksProcMacro::new(callbacks)).unwrap();
  Capture::new_proc_macro("exint_macro", root)
});

static EXINT: LazyLock<Capture> = LazyLock::new(|| {
  let root: PathBuf = CONTEXT.root_path().join("src/lib.rs");
  let deps: PathBuf = CONTEXT.deps_path();

  let mut callbacks: CallbacksDefault = CallbacksDefault::new("exint", &root, &deps);

  callbacks.dependency(&deps, &EXINT_MACRO);
  callbacks.features(FEATURES.iter().copied());

  Capture::rustc(&mut CallbacksRLib::new(callbacks)).unwrap();
  Capture::new_rlib("exint", root)
});

pub(crate) struct TestCase {
  pub(crate) name: &'static str,
  pub(crate) path: &'static str,
  pub(crate) file: &'static str,
}

impl TestCase {
  pub(crate) fn desc(&self) -> TestDesc {
    TestDesc {
      name: StaticTestName(self.name),
      ignore: FILECHECK.is_none(),
      ignore_message: None,
      source_file: file!(),
      start_line: 0,
      start_col: 0,
      end_line: 0,
      end_col: 0,
      compile_fail: false,
      no_run: false,
      should_panic: ShouldPanic::No,
      test_type: TestType::IntegrationTest,
    }
  }

  pub(crate) fn run(&self) {
    let root: PathBuf = CONTEXT.root_path().join(self.path);
    let deps: PathBuf = CONTEXT.deps_path();

    let mut callbacks: CallbacksDefault = CallbacksDefault::new(self.file, &root, &deps);

    callbacks.dependency(&deps, &EXINT);

    let compile_and_test = || -> Result<(), Error> {
      Capture::rustc(&mut CallbacksAssembly::new(callbacks))?;
      Capture::check(Capture::new_rlib(self.file, root), &CONTEXT)
    };

    if let Err(error) = compile_and_test() {
      panic!("{error}");
    }
  }
}
