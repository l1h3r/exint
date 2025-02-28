#![feature(f128)]

use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef fp128 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i8 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i16 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i24 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i32 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i40 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i48 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i56 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> f128 {
  // CHECK: %[[reg:.*]] = sitofp i64 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i72 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i80 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i88 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i96 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i104 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = sitofp i112 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i8 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i16 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i24 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i32 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i40 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i48 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i56 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> f128 {
  // CHECK: %[[reg:.*]] = uitofp i64 {{.*}} to fp128
  // CHECK: ret fp128 %[[reg]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i72 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i80 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i88 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i96 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i104 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}

// CHECK-LABEL: define noundef fp128 @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> f128 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = uitofp i112 %[[reg_a]] to fp128
  // CHECK: ret fp128 %[[reg_b]]
  a.into()
}
