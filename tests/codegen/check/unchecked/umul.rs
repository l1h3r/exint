use exint::uint;

// CHECK-LABEL: define i8 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> uint<1> {
  // CHECK: %[[C:.+]] = mul nuw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i16 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> uint<2> {
  // CHECK: %[[C:.+]] = mul nuw i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i24 @uint_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> uint<3> {
  // CHECK: %[[C:.+]] = mul i24 %[[B]], %[[A]]
  // CHECK: ret i24 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i32 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> uint<4> {
  // CHECK: %[[C:.+]] = mul nuw i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i40 @uint_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> uint<5> {
  // CHECK: %[[C:.+]] = mul i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i48 @uint_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> uint<6> {
  // CHECK: %[[C:.+]] = mul i48 %[[B]], %[[A]]
  // CHECK: ret i48 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i56 @uint_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> uint<7> {
  // CHECK: %[[C:.+]] = mul i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define i64 @uint_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> uint<8> {
  // CHECK: %[[C:.+]] = mul nuw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_9
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> uint<9> {
  // CHECK: %[[D:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i72 %[[E]], %[[D]]
  // CHECK: store i72 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_10
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> uint<10> {
  // CHECK: %[[D:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i80 %[[E]], %[[D]]
  // CHECK: store i80 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_11
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> uint<11> {
  // CHECK: %[[D:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i88 %[[E]], %[[D]]
  // CHECK: store i88 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_12
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> uint<12> {
  // CHECK: %[[D:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i96 %[[E]], %[[D]]
  // CHECK: store i96 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_13
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> uint<13> {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_14
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> uint<14> {
  // CHECK: %[[D:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i112 %[[E]], %[[D]]
  // CHECK: store i112 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_15
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> uint<15> {
  // CHECK: %[[D:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i120 %[[E]], %[[D]]
  // CHECK: store i120 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}

// CHECK-LABEL: define void @uint_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> uint<16> {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul nuw i128 %[[E]], %[[D]]
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { a.unchecked_mul(b) }
}
