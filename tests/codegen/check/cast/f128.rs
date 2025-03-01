#![feature(f128)]

use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef fp128 @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i8 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i16 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i24 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i32 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i40 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i48 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i56 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> f128 {
  // CHECK: %[[B:.+]] = sitofp i64 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> f128 {
  // CHECK: %[[B:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i72 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> f128 {
  // CHECK: %[[B:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i80 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> f128 {
  // CHECK: %[[B:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i88 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> f128 {
  // CHECK: %[[B:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i96 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> f128 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i104 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> f128 {
  // CHECK: %[[B:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = sitofp i112 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i8 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i16 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i24 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i32 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i40 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i48 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i56 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> f128 {
  // CHECK: %[[B:.+]] = uitofp i64 %[[A]] to fp128
  // CHECK: ret fp128 %[[B]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> f128 {
  // CHECK: %[[B:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i72 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> f128 {
  // CHECK: %[[B:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i80 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> f128 {
  // CHECK: %[[B:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i88 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> f128 {
  // CHECK: %[[B:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i96 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> f128 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i104 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> f128 {
  // CHECK: %[[B:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = uitofp i112 %[[B]] to fp128
  // CHECK: ret fp128 %[[C]]
  a.into()
}
