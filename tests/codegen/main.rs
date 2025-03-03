use compiletest::filecheck;

// TODO: Split into multiple #[test] fns
#[test]
fn test_codegen() {
  filecheck::purge();

  filecheck::verify("codegen/check/bitwise/and.rs");
  filecheck::verify("codegen/check/bitwise/or.rs");
  filecheck::verify("codegen/check/bitwise/xor.rs");
  filecheck::verify("codegen/check/bitwise/not.rs");
  filecheck::verify("codegen/check/bitwise/dor.rs");

  filecheck::verify("codegen/check/cast/f16.rs");
  filecheck::verify("codegen/check/cast/f32.rs");
  filecheck::verify("codegen/check/cast/f64.rs");
  filecheck::verify("codegen/check/cast/f128.rs");

  filecheck::verify("codegen/check/compare/eq.rs");
  filecheck::verify("codegen/check/compare/ucmp.rs");
  filecheck::verify("codegen/check/compare/scmp.rs");

  filecheck::verify("codegen/check/convert/swap1.rs");
  filecheck::verify("codegen/check/convert/swap8.rs");
  filecheck::verify("codegen/check/convert/rotl.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/convert/rotr.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120

  filecheck::verify("codegen/check/inspect/ctpop.rs");
  filecheck::verify("codegen/check/inspect/ctlz.rs");
  filecheck::verify("codegen/check/inspect/cttz.rs");
  filecheck::verify("codegen/check/inspect/ctlz_nonzero.rs");
  filecheck::verify("codegen/check/inspect/cttz_nonzero.rs");

  filecheck::verify("codegen/check/overflowing/uadd.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/overflowing/usub.rs");
  filecheck::verify("codegen/check/overflowing/umul.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120

  filecheck::verify("codegen/check/overflowing/sadd.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/overflowing/ssub.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/overflowing/smul.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120

  filecheck::verify("codegen/check/saturating/uadd.rs");
  filecheck::verify("codegen/check/saturating/usub.rs");
  filecheck::verify("codegen/check/saturating/sadd.rs"); // TODO: i24,i40,i48,i56
  filecheck::verify("codegen/check/saturating/ssub.rs"); // TODO: i24,i40,i48,i56

  filecheck::verify("codegen/check/unchecked/uadd.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/usub.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/umul.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/udiv.rs");
  filecheck::verify("codegen/check/unchecked/urem.rs");

  filecheck::verify("codegen/check/unchecked/sadd.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/ssub.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/smul.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/sdiv.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/srem.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120

  filecheck::verify("codegen/check/unchecked/shl.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/lshr.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120
  filecheck::verify("codegen/check/unchecked/ashr.rs"); // TODO: i24,i40,i48,i56,i72,i80,i88,i96,i104,i112,i120

  filecheck::verify("codegen/check/wrapping/add.rs");
  filecheck::verify("codegen/check/wrapping/sub.rs");
  filecheck::verify("codegen/check/wrapping/mul.rs");
}
