use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef zeroext i1 @int_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i8 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i16 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i24 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i32 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i40 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i48 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i56 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i64 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> bool {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i72 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> bool {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i80 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> bool {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i88 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> bool {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i96 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> bool {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i104 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> bool {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i112 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> bool {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i120 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> bool {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i128 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i8 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i16 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i24 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i32 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i40 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i48 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i56 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_8
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i64 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> bool {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i72 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> bool {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i80 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> bool {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i88 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> bool {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i96 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> bool {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i104 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> bool {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i112 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> bool {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i120 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> bool {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i128 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  a == b
}
