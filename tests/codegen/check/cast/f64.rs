use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef double @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i16 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i24 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i32 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i40 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @int_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> f64 {
  // CHECK: %[[B:.+]] = sitofp i48 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i16 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i24 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i32 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i40 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef double @uint_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> f64 {
  // CHECK: %[[B:.+]] = uitofp i48 %[[A]] to double
  // CHECK: ret double %[[B]]
  a.into()
}
