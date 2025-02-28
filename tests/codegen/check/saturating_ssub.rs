use exint::int;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ssub.sat.i8(i8 %{{.*}}, i8 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[reg:.*]] = tail call i16 @llvm.ssub.sat.i16(i16 %{{.*}}, i16 %{{.*}})
  // CHECK: ret i16 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[reg:.*]] = tail call i32 @llvm.ssub.sat.i32(i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[reg:.*]] = tail call i64 @llvm.ssub.sat.i64(i64 %{{.*}}, i64 %{{.*}})
  // CHECK: ret i64 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i128 @llvm.ssub.sat.i128(i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}
