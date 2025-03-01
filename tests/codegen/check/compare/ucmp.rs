use exint::uint;

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i24(i24 %[[A]], i24 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i48(i48 %[[A]], i48 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i72(i72 %[[C]], i72 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i80(i80 %[[C]], i80 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i88(i88 %[[C]], i88 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i96(i96 %[[C]], i96 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i104(i104 %[[C]], i104 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i112(i112 %[[C]], i112 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i120(i120 %[[C]], i120 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call i8 @llvm.ucmp.i8.i128(i128 %[[C]], i128 %[[D]])
  // CHECK: ret i8 %[[E]]
  a.cmp(&b)
}
