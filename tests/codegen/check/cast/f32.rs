use exint::int;
use exint::uint;

// -----------------------------------------------------------------------------
// core - signed
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @i8
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn i8(a: i8) -> f32 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef float @i16
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn i16(a: i16) -> f32 {
  // CHECK: %[[B:.+]] = sitofp i16 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// core - unsigned
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @u8
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn u8(a: u8) -> f32 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef float @u16
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn u16(a: u16) -> f32 {
  // CHECK: %[[B:.+]] = uitofp i16 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - standard - signed
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f32 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef float @int_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f32 {
  // CHECK: %[[B:.+]] = sitofp i16 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - standard - unsigned
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f32 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef float @uint_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f32 {
  // CHECK: %[[B:.+]] = uitofp i16 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - extended - signed
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @int_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f32 {
  // CHECK: %[[B:.+]] = sitofp i24 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}

// -----------------------------------------------------------------------------
// exint - extended - unsigned
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef float @uint_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f32 {
  // CHECK: %[[B:.+]] = uitofp i24 %[[A]] to float
  // CHECK: ret float %[[B]]
  a.into()
}
