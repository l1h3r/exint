use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef float @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f32 {
  // CHECK: %[[reg:.*]] = sitofp i8 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef float @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f32 {
  // CHECK: %[[reg:.*]] = sitofp i16 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef float @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f32 {
  // CHECK: %[[reg:.*]] = sitofp i24 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef float @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f32 {
  // CHECK: %[[reg:.*]] = uitofp i8 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef float @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f32 {
  // CHECK: %[[reg:.*]] = uitofp i16 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef float @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f32 {
  // CHECK: %[[reg:.*]] = uitofp i24 {{.*}} to float
  // CHECK: ret float %[[reg]]
  a.into()
}
