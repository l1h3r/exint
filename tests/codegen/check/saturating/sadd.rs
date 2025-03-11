use exint::int;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> int<1> {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.sadd.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.saturating_add(b)
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> int<2> {
  // CHECK: %[[C:.+]] = tail call i16 @llvm.sadd.sat.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i16 %[[C]]
  a.saturating_add(b)
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> int<4> {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.sadd.sat.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  a.saturating_add(b)
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> int<8> {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.sadd.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_9
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> int<9> {
  // CHECK: %[[D:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i72 @llvm.sadd.sat.i72(i72 %[[E]], i72 %[[D]])
  // CHECK: store i72 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_10
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> int<10> {
  // CHECK: %[[D:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i80 @llvm.sadd.sat.i80(i80 %[[E]], i80 %[[D]])
  // CHECK: store i80 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_11
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> int<11> {
  // CHECK: %[[D:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i88 @llvm.sadd.sat.i88(i88 %[[E]], i88 %[[D]])
  // CHECK: store i88 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_12
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> int<12> {
  // CHECK: %[[D:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i96 @llvm.sadd.sat.i96(i96 %[[E]], i96 %[[D]])
  // CHECK: store i96 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_13
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> int<13> {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i104 @llvm.sadd.sat.i104(i104 %[[E]], i104 %[[D]])
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_14
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> int<14> {
  // CHECK: %[[D:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i112 @llvm.sadd.sat.i112(i112 %[[E]], i112 %[[D]])
  // CHECK: store i112 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_15
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> int<15> {
  // CHECK: %[[D:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i120 @llvm.sadd.sat.i120(i120 %[[E]], i120 %[[D]])
  // CHECK: store i120 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}

// CHECK-LABEL: define void @int_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> int<16> {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i128 @llvm.sadd.sat.i128(i128 %[[D]], i128 %[[E]])
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  a.saturating_add(b)
}
