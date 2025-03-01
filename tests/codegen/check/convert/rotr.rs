use exint::int;
use exint::uint;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: u32) -> int<1> {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: u32) -> int<2> {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = tail call i16 @llvm.fshr.i16(i16 %[[A]], i16 %[[A]], i16 %[[C]])
  // CHECK: ret i16 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: u32) -> int<4> {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.fshr.i32(i32 %[[A]], i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: u32) -> int<8> {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: u32) -> int<16> {
  // CHECK: %[[reg_a:.+]] = load i128, ptr %{{.+}}, align 1
  // CHECK: %[[reg_b:.+]] = zext i32 %{{.+}} to i128
  // CHECK: %[[reg_c:.+]] = tail call i128 @llvm.fshr.i128(i128 %[[reg_a]], i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.+}}, align 1
  // CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: define i8 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: u32) -> uint<1> {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i16 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: u32) -> uint<2> {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = tail call i16 @llvm.fshr.i16(i16 %[[A]], i16 %[[A]], i16 %[[C]])
  // CHECK: ret i16 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i32 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: u32) -> uint<4> {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.fshr.i32(i32 %[[A]], i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  a.rotate_right(b)
}

// CHECK-LABEL: define i64 @uint_8
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: u32) -> uint<8> {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  a.rotate_right(b)
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: u32) -> uint<16> {
  // CHECK: %[[reg_a:.+]] = load i128, ptr %{{.+}}, align 1
  // CHECK: %[[reg_b:.+]] = zext i32 %{{.+}} to i128
  // CHECK: %[[reg_c:.+]] = tail call i128 @llvm.fshr.i128(i128 %[[reg_a]], i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.+}}, align 1
  // CHECK: ret void
  a.rotate_right(b)
}
