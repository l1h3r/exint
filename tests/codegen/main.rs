//! Exint Codegen Tests

#![cfg_attr(not(miri), feature(rustc_private))]
#![feature(test)]

#[cfg(not(miri))] extern crate rustc_driver;
#[cfg(not(miri))] extern crate rustc_interface;
#[cfg(not(miri))] extern crate rustc_lint_defs;
#[cfg(not(miri))] extern crate rustc_session;
#[cfg(not(miri))] extern crate rustc_span;
#[cfg(not(miri))] extern crate rustc_target;

extern crate test;

#[cfg(not(miri))]
mod compile;

#[cfg(not(miri))]
mod harness;

#[cfg(not(miri))]
macro_rules! check {
  ($name:ident, $path:literal) => {{
    const CASE: &'static $crate::harness::TestCase = &$crate::harness::TestCase {
      name: concat!("codegen::", stringify!($name)),
      path: concat!("tests/codegen/check/", $path),
      file: stringify!($name),
    };

    &::test::TestDescAndFn {
      desc: CASE.desc(),
      testfn: ::test::StaticTestFn(|| ::test::assert_test_result(CASE.run())),
    }
  }};
}

#[cfg(miri)]
macro_rules! check {
  ($name:ident, $path:literal) => {
    &::test::TestDescAndFn {
      desc: ::test::TestDesc {
        name: ::test::StaticTestName(concat!("codegen::", stringify!($name))),
        ignore: true,
        ignore_message: None,
        source_file: file!(),
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        compile_fail: false,
        no_run: false,
        should_panic: ::test::ShouldPanic::No,
        test_type: ::test::TestType::IntegrationTest,
      },
      testfn: ::test::StaticTestFn(|| Ok(())),
    }
  };
}

fn main() {
  let tests: &[&test::TestDescAndFn] = &[
    check!(test_cast_f16, "cast/f16.rs"),
    check!(test_cast_f32, "cast/f32.rs"),
    check!(test_cast_f64, "cast/f64.rs"),
    check!(test_cast_f128, "cast/f128.rs"),

    check!(test_standard_u8, "standard/u8.rs"),
    check!(test_standard_u16, "standard/u16.rs"),
    check!(test_standard_u32, "standard/u32.rs"),
    check!(test_standard_u64, "standard/u64.rs"),
    check!(test_standard_u128, "standard/u128.rs"),

    check!(test_extended_small_u8, "extended/small/u8.rs"),
    check!(test_extended_small_u16, "extended/small/u16.rs"),
    check!(test_extended_small_u24, "extended/small/u24.rs"),
    check!(test_extended_small_u32, "extended/small/u32.rs"),
    check!(test_extended_small_u40, "extended/small/u40.rs"),
    check!(test_extended_small_u48, "extended/small/u48.rs"),
    check!(test_extended_small_u56, "extended/small/u56.rs"),
    check!(test_extended_small_u64, "extended/small/u64.rs"),

    check!(test_extended_large_u72, "extended/large/u72.rs"),
    check!(test_extended_large_u80, "extended/large/u80.rs"),
    check!(test_extended_large_u88, "extended/large/u88.rs"),
    check!(test_extended_large_u96, "extended/large/u96.rs"),
    check!(test_extended_large_u104, "extended/large/u104.rs"),
    check!(test_extended_large_u112, "extended/large/u112.rs"),
    check!(test_extended_large_u120, "extended/large/u120.rs"),
    check!(test_extended_large_u128, "extended/large/u128.rs"),
  ];

  test::test_main_static(tests)
}
