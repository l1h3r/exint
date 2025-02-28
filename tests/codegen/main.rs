use compiletest::filecheck;

// TODO: Split into multiple #[test] fns
#[test]
fn test_codegen() {
  filecheck::purge();

  filecheck::verify("codegen/check/bitwise_and.rs");
  filecheck::verify("codegen/check/bitwise_or.rs");
  filecheck::verify("codegen/check/bitwise_xor.rs");
  filecheck::verify("codegen/check/bitwise_not.rs");

  filecheck::verify("codegen/check/cast_f16.rs");
  filecheck::verify("codegen/check/cast_f32.rs");
  filecheck::verify("codegen/check/cast_f64.rs");
  filecheck::verify("codegen/check/cast_f128.rs");

  filecheck::verify("codegen/check/compare_eq.rs");
  filecheck::verify("codegen/check/compare_ucmp.rs");
  filecheck::verify("codegen/check/compare_scmp.rs");

  filecheck::verify("codegen/check/convert_swap1.rs");
  filecheck::verify("codegen/check/convert_swap8.rs");
  filecheck::verify("codegen/check/convert_rotl.rs"); // TODO
  filecheck::verify("codegen/check/convert_rotr.rs"); // TODO

  filecheck::verify("codegen/check/inspect_ctpop.rs");
  filecheck::verify("codegen/check/inspect_ctlz.rs");
  filecheck::verify("codegen/check/inspect_cttz.rs");
  filecheck::verify("codegen/check/inspect_ctlz_nonzero.rs");
  filecheck::verify("codegen/check/inspect_cttz_nonzero.rs");

  filecheck::verify("codegen/check/saturating_uadd.rs");
  filecheck::verify("codegen/check/saturating_usub.rs");
  filecheck::verify("codegen/check/saturating_sadd.rs"); // TODO
  filecheck::verify("codegen/check/saturating_ssub.rs"); // TODO

  filecheck::verify("codegen/check/unchecked_uadd.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_usub.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_umul.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_udiv.rs");
  filecheck::verify("codegen/check/unchecked_urem.rs");
  filecheck::verify("codegen/check/unchecked_sadd.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_ssub.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_smul.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_sdiv.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_srem.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_shl.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_lshr.rs"); // TODO
  filecheck::verify("codegen/check/unchecked_ashr.rs"); // TODO

  filecheck::verify("codegen/check/wrapping_add.rs");
  filecheck::verify("codegen/check/wrapping_sub.rs");
  filecheck::verify("codegen/check/wrapping_mul.rs");
}
