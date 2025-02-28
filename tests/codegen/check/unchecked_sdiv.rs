use exint::int;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[reg:.*]] = sdiv i8 %{{.*}}, %{{.*}}
  // CHECK: ret i8 %[[reg]]
  unsafe { exint::backend::unchecked_sdiv::<int<1>, 1>(a, b) }
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[reg:.*]] = sdiv i16 %{{.*}}, %{{.*}}
  // CHECK: ret i16 %[[reg]]
  unsafe { exint::backend::unchecked_sdiv::<int<2>, 2>(a, b) }
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[reg:.*]] = sdiv i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { exint::backend::unchecked_sdiv::<int<4>, 4>(a, b) }
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[reg:.*]] = sdiv i64 %{{.*}}, %{{.*}}
  // CHECK: ret i64 %[[reg]]
  unsafe { exint::backend::unchecked_sdiv::<int<8>, 8>(a, b) }
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = sdiv i128 %[[reg_a]], %[[reg_b]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<16>, 16>(a, b) }
}
