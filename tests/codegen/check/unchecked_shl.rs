use exint::int;
use exint::uint;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: u32) -> int<1> {
  // CHECK: %[[reg_a:.*]] = trunc nuw i32 %{{.*}} to i8
  // CHECK: %[[reg_b:.*]] = shl i8 %{{.*}}, %[[reg_a]]
  // CHECK: ret i8 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: u32) -> int<2> {
  // CHECK: %[[reg_a:.*]] = trunc nuw i32 %{{.*}} to i16
  // CHECK: %[[reg_b:.*]] = shl i16 %{{.*}}, %[[reg_a]]
  // CHECK: ret i16 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: u32) -> int<4> {
  // CHECK: %[[reg:.*]] = shl i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: u32) -> int<8> {
  // CHECK: %[[reg_a:.*]] = zext nneg i32 %{{.*}} to i64
  // CHECK: %[[reg_b:.*]] = shl i64 %{{.*}}, %[[reg_a]]
  // CHECK: ret i64 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: u32) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = zext nneg i32 %{{.*}} to i128
  // CHECK: %[[reg_c:.*]] = shl i128 %[[reg_a]], %[[reg_b]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: u32) -> uint<1> {
  // CHECK: %[[reg_a:.*]] = trunc nuw i32 %{{.*}} to i8
  // CHECK: %[[reg_b:.*]] = shl i8 %{{.*}}, %[[reg_a]]
  // CHECK: ret i8 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: u32) -> uint<2> {
  // CHECK: %[[reg_a:.*]] = trunc nuw i32 %{{.*}} to i16
  // CHECK: %[[reg_b:.*]] = shl i16 %{{.*}}, %[[reg_a]]
  // CHECK: ret i16 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: u32) -> uint<4> {
  // CHECK: %[[reg:.*]] = shl i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: u32) -> uint<8> {
  // CHECK: %[[reg_a:.*]] = zext nneg i32 %{{.*}} to i64
  // CHECK: %[[reg_b:.*]] = shl i64 %{{.*}}, %[[reg_a]]
  // CHECK: ret i64 %[[reg_b]]
  unsafe { a.unchecked_shl(b) }
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: u32) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = zext nneg i32 %{{.*}} to i128
  // CHECK: %[[reg_c:.*]] = shl i128 %[[reg_a]], %[[reg_b]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { a.unchecked_shl(b) }
}
