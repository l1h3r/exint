use exint::int;

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i24(i24 %[[A]], i24 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i48(i48 %[[A]], i48 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i72(i72 %[[C]], i72 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i80(i80 %[[C]], i80 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i88(i88 %[[C]], i88 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i96(i96 %[[C]], i96 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i104(i104 %[[C]], i104 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i112(i112 %[[C]], i112 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i120(i120 %[[C]], i120 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.scmp.i8.i128(i128 %[[C]], i128 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}
