use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef double @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i8 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i16 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i24 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i32 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i40 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> f64 {
  // CHECK: %[[reg:.*]] = sitofp i48 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i8 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i16 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i24 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i32 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i40 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> f64 {
  // CHECK: %[[reg:.*]] = uitofp i48 {{.*}} to double
  // CHECK: ret double %[[reg]]
  a.into()
}
