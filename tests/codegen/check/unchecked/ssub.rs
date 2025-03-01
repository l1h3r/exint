use exint::int;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[C:.+]] = sub nsw i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[C:.+]] = sub nsw i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i24 @int_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> int<3> {
  // CHECK: %[[C:.+]] = sub i24 %[[A]], %[[B]]
  // CHECK: ret i24 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[C:.+]] = sub nsw i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i40 @int_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> int<5> {
  // CHECK: %[[C:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i48 @int_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> int<6> {
  // CHECK: %[[C:.+]] = sub i48 %[[A]], %[[B]]
  // CHECK: ret i48 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i56 @int_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> int<7> {
  // CHECK: %[[C:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[C:.+]] = sub nsw i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_9
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> int<9> {
  // CHECK: %[[D:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i72 %[[D]], %[[E]]
  // CHECK: store i72 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_10
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> int<10> {
  // CHECK: %[[D:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i80 %[[D]], %[[E]]
  // CHECK: store i80 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_11
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> int<11> {
  // CHECK: %[[D:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i88 %[[D]], %[[E]]
  // CHECK: store i88 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_12
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> int<12> {
  // CHECK: %[[D:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i96 %[[D]], %[[E]]
  // CHECK: store i96 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_13
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> int<13> {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_14
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> int<14> {
  // CHECK: %[[D:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i112 %[[D]], %[[E]]
  // CHECK: store i112 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_15
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> int<15> {
  // CHECK: %[[D:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i120 %[[D]], %[[E]]
  // CHECK: store i120 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}

// CHECK-LABEL: define void @int_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub nsw i128 %[[D]], %[[E]]
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_sub(b) }
}
