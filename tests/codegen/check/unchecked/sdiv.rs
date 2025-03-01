use exint::int;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[C:.+]] = sdiv i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<int<1>, 1>(a, b) }
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[C:.+]] = sdiv i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<int<2>, 2>(a, b) }
}

// CHECK-LABEL: define i24 @int_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> int<3> {
  // CHECK: %[[C:.+]] = sext i24 %[[A]] to i32
  // CHECK: %[[D:.+]] = sext i24 %[[B]] to i32
  // CHECK: %[[E:.+]] = sdiv i32 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i32 %[[E]] to i24
  // CHECK: ret i24 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<int<3>, 3>(a, b) }
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[C:.+]] = sdiv i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<int<4>, 4>(a, b) }
}

// CHECK-LABEL: define i40 @int_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> int<5> {
  // CHECK: %[[C:.+]] = sext i40 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i40 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i40
  // CHECK: ret i40 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<int<5>, 5>(a, b) }
}

// CHECK-LABEL: define i48 @int_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> int<6> {
  // CHECK: %[[C:.+]] = sext i48 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i48 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i48
  // CHECK: ret i48 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<int<6>, 6>(a, b) }
}

// CHECK-LABEL: define i56 @int_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> int<7> {
  // CHECK: %[[C:.+]] = sext i56 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i56 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i56
  // CHECK: ret i56 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<int<7>, 7>(a, b) }
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[C:.+]] = sdiv i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<int<8>, 8>(a, b) }
}

// CHECK-LABEL: define void @int_9
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> int<9> {
  // CHECK: %[[D:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i72 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i72 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i72
  // CHECK: store i72 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<9>, 9>(a, b) }
}

// CHECK-LABEL: define void @int_10
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> int<10> {
  // CHECK: %[[D:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i80 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i80 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i80
  // CHECK: store i80 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<10>, 10>(a, b) }
}

// CHECK-LABEL: define void @int_11
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> int<11> {
  // CHECK: %[[D:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i88 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i88 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i88
  // CHECK: store i88 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<11>, 11>(a, b) }
}

// CHECK-LABEL: define void @int_12
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> int<12> {
  // CHECK: %[[D:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i96 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i96 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i96
  // CHECK: store i96 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<12>, 12>(a, b) }
}

// CHECK-LABEL: define void @int_13
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> int<13> {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i104 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i104 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i104
  // CHECK: store i104 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<13>, 13>(a, b) }
}

// CHECK-LABEL: define void @int_14
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> int<14> {
  // CHECK: %[[D:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i112 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i112 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i112
  // CHECK: store i112 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<14>, 14>(a, b) }
}

// CHECK-LABEL: define void @int_15
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> int<15> {
  // CHECK: %[[D:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i120 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i120 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i120
  // CHECK: store i120 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<15>, 15>(a, b) }
}

// CHECK-LABEL: define void @int_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sdiv i128 %[[D]], %[[E]]
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<int<16>, 16>(a, b) }
}
