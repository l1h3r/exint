#![feature(f16)]
#![feature(f128)]

use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef half @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f16 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef half @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f16 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to half
  // CHECK: ret half %[[B]]
  a.into()
}
