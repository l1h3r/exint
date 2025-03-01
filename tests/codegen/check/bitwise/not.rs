use exint::int;
use exint::uint;

// CHECK-LABEL: define i8 @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> int<1> {
  // CHECK: %[[B:.+]] = xor i8 %[[A]], -1
  // CHECK: ret i8 %[[B]]
  !a
}

// CHECK-LABEL: define i16 @int_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> int<2> {
  // CHECK: %[[B:.+]] = xor i16 %[[A]], -1
  // CHECK: ret i16 %[[B]]
  !a
}

// CHECK-LABEL: define i24 @int_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> int<3> {
  // CHECK: %[[B:.+]] = xor i24 %[[A]], -1
  // CHECK: ret i24 %[[B]]
  !a
}

// CHECK-LABEL: define i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> int<4> {
  // CHECK: %[[B:.+]] = xor i32 %[[A]], -1
  // CHECK: ret i32 %[[B]]
  !a
}

// CHECK-LABEL: define i40 @int_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> int<5> {
  // CHECK: %[[B:.+]] = xor i40 %[[A]], -1
  // CHECK: ret i40 %[[B]]
  !a
}

// CHECK-LABEL: define i48 @int_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> int<6> {
  // CHECK: %[[B:.+]] = xor i48 %[[A]], -1
  // CHECK: ret i48 %[[B]]
  !a
}

// CHECK-LABEL: define i56 @int_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> int<7> {
  // CHECK: %[[B:.+]] = xor i56 %[[A]], -1
  // CHECK: ret i56 %[[B]]
  !a
}

// CHECK-LABEL: define i64 @int_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> int<8> {
  // CHECK: %[[B:.+]] = xor i64 %[[A]], -1
  // CHECK: ret i64 %[[B]]
  !a
}

// CHECK-LABEL: define void @int_9
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> int<9> {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i72 %[[C]], -1
  // CHECK: store i72 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_10
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> int<10> {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i80 %[[C]], -1
  // CHECK: store i80 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_11
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> int<11> {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i88 %[[C]], -1
  // CHECK: store i88 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_12
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> int<12> {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i96 %[[C]], -1
  // CHECK: store i96 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_13
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> int<13> {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i104 %[[C]], -1
  // CHECK: store i104 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_14
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> int<14> {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i112 %[[C]], -1
  // CHECK: store i112 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_15
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>) -> int<15> {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i120 %[[C]], -1
  // CHECK: store i120 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_16
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>) -> int<16> {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i128 %[[C]], -1
  // CHECK: store i128 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define i8 @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> uint<1> {
  // CHECK: %[[B:.+]] = xor i8 %[[A]], -1
  // CHECK: ret i8 %[[B]]
  !a
}

// CHECK-LABEL: define i16 @uint_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> uint<2> {
  // CHECK: %[[B:.+]] = xor i16 %[[A]], -1
  // CHECK: ret i16 %[[B]]
  !a
}

// CHECK-LABEL: define i24 @uint_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> uint<3> {
  // CHECK: %[[B:.+]] = xor i24 %[[A]], -1
  // CHECK: ret i24 %[[B]]
  !a
}

// CHECK-LABEL: define i32 @uint_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> uint<4> {
  // CHECK: %[[B:.+]] = xor i32 %[[A]], -1
  // CHECK: ret i32 %[[B]]
  !a
}

// CHECK-LABEL: define i40 @uint_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> uint<5> {
  // CHECK: %[[B:.+]] = xor i40 %[[A]], -1
  // CHECK: ret i40 %[[B]]
  !a
}

// CHECK-LABEL: define i48 @uint_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> uint<6> {
  // CHECK: %[[B:.+]] = xor i48 %[[A]], -1
  // CHECK: ret i48 %[[B]]
  !a
}

// CHECK-LABEL: define i56 @uint_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> uint<7> {
  // CHECK: %[[B:.+]] = xor i56 %[[A]], -1
  // CHECK: ret i56 %[[B]]
  !a
}

// CHECK-LABEL: define i64 @uint_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> uint<8> {
  // CHECK: %[[B:.+]] = xor i64 %[[A]], -1
  // CHECK: ret i64 %[[B]]
  !a
}

// CHECK-LABEL: define void @uint_9
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> uint<9> {
  // CHECK: %[[C:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i72 %[[C]], -1
  // CHECK: store i72 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_10
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> uint<10> {
  // CHECK: %[[C:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i80 %[[C]], -1
  // CHECK: store i80 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_11
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> uint<11> {
  // CHECK: %[[C:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i88 %[[C]], -1
  // CHECK: store i88 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_12
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> uint<12> {
  // CHECK: %[[C:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i96 %[[C]], -1
  // CHECK: store i96 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_13
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> uint<13> {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i104 %[[C]], -1
  // CHECK: store i104 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_14
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> uint<14> {
  // CHECK: %[[C:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i112 %[[C]], -1
  // CHECK: store i112 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_15
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>) -> uint<15> {
  // CHECK: %[[C:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i120 %[[C]], -1
  // CHECK: store i120 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_16
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>) -> uint<16> {
  // CHECK: %[[C:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i128 %[[C]], -1
  // CHECK: store i128 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  !a
}
