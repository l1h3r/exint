#![feature(f16)]

use exint::int;
use exint::uint;

// -----------------------------------------------------------------------------
// core - signed
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef half @i8
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn i8(a: i8) -> f16 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// core - unsigned
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef half @u8
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn u8(a: u8) -> f16 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - standard - signed
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef half @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f16 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - standard - unsigned
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef half @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f16 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - extended - signed
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// exint - extended - unsigned
// -----------------------------------------------------------------------------
