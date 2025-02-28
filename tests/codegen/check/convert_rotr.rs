use exint::int;
use exint::uint;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: u32) -> int<1> {
  // CHECK: %[[reg_a:.*]] = trunc i32 %{{.*}} to i8
  // CHECK: %[[reg_b:.*]] = tail call i8 @llvm.fshr.i8(i8 %{{.*}}, i8 %{{.*}}, i8 %[[reg_a]])
  // CHECK: ret i8 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: u32) -> int<2> {
  // CHECK: %[[reg_a:.*]] = trunc i32 %{{.*}} to i16
  // CHECK: %[[reg_b:.*]] = tail call i16 @llvm.fshr.i16(i16 %{{.*}}, i16 %{{.*}}, i16 %[[reg_a]])
  // CHECK: ret i16 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: u32) -> int<4> {
  // CHECK: %[[reg:.*]] = tail call i32 @llvm.fshr.i32(i32 %{{.*}}, i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: u32) -> int<8> {
  // CHECK: %[[reg_a:.*]] = zext i32 %{{.*}} to i64
  // CHECK: %[[reg_b:.*]] = tail call i64 @llvm.fshr.i64(i64 %{{.*}}, i64 %{{.*}}, i64 %[[reg_a]])
  // CHECK: ret i64 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: u32) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = zext i32 %{{.*}} to i128
  // CHECK: %[[reg_c:.*]] = tail call i128 @llvm.fshr.i128(i128 %[[reg_a]], i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: u32) -> uint<1> {
  // CHECK: %[[reg_a:.*]] = trunc i32 %{{.*}} to i8
  // CHECK: %[[reg_b:.*]] = tail call i8 @llvm.fshr.i8(i8 %{{.*}}, i8 %{{.*}}, i8 %[[reg_a]])
  // CHECK: ret i8 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: u32) -> uint<2> {
  // CHECK: %[[reg_a:.*]] = trunc i32 %{{.*}} to i16
  // CHECK: %[[reg_b:.*]] = tail call i16 @llvm.fshr.i16(i16 %{{.*}}, i16 %{{.*}}, i16 %[[reg_a]])
  // CHECK: ret i16 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: u32) -> uint<4> {
  // CHECK: %[[reg:.*]] = tail call i32 @llvm.fshr.i32(i32 %{{.*}}, i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: u32) -> uint<8> {
  // CHECK: %[[reg_a:.*]] = zext i32 %{{.*}} to i64
  // CHECK: %[[reg_b:.*]] = tail call i64 @llvm.fshr.i64(i64 %{{.*}}, i64 %{{.*}}, i64 %[[reg_a]])
  // CHECK: ret i64 %[[reg_b]]
  a.rotate_right(b)
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: u32) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = zext i32 %{{.*}} to i128
  // CHECK: %[[reg_c:.*]] = tail call i128 @llvm.fshr.i128(i128 %[[reg_a]], i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.rotate_right(b)
}
