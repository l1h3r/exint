use exint::int;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[reg:.*]] = sub nsw i8 %{{.*}}, %{{.*}}
  // CHECK: ret i8 %[[reg]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[reg:.*]] = sub nsw i16 %{{.*}}, %{{.*}}
  // CHECK: ret i16 %[[reg]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[reg:.*]] = sub nsw i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[reg:.*]] = sub nsw i64 %{{.*}}, %{{.*}}
  // CHECK: ret i64 %[[reg]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = sub nsw i128 %[[reg_a]], %[[reg_b]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}
