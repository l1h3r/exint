use exint::int;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: u32) -> int<1> {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = ashr i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { a.unchecked_shr(b) }
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: u32) -> int<2> {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = ashr i16 %[[A]], %[[C]]
  // CHECK: ret i16 %[[D]]
  unsafe { a.unchecked_shr(b) }
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: u32) -> int<4> {
  // CHECK: %[[C:.+]] = ashr i32 %[[A]], %{{.*}}
  // CHECK: ret i32 %[[C]]
  unsafe { a.unchecked_shr(b) }
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: u32) -> int<8> {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = ashr i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { a.unchecked_shr(b) }
}

// CHECK-LABEL: define void @int_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: u32) -> int<16> {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = zext nneg i32 %[[B]] to i128
  // CHECK: %[[F:.+]] = ashr i128 %[[D]], %[[E]]
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_shr(b) }
}
