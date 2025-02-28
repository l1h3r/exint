#![feature(f16)]
#![feature(f128)]

use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef half @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f16 {
  // CHECK: %[[reg:.*]] = sitofp i8 {{.*}} to half
  // CHECK: ret half %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef half @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f16 {
  // CHECK: %[[reg:.*]] = uitofp i8 {{.*}} to half
  // CHECK: ret half %[[reg]]
  a.into()
}
