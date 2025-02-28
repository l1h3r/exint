use exint::int;
use exint::uint;

// CHECK-LABEL: define i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> int<1> {
  // CHECK: %[[reg:.*]] = xor i8 %{{.*}}, -1
  // CHECK: ret i8 %[[reg]]
  !a
}

// CHECK-LABEL: define i16 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> int<2> {
  // CHECK: %[[reg:.*]] = xor i16 %{{.*}}, -1
  // CHECK: ret i16 %[[reg]]
  !a
}

// CHECK-LABEL: define i24 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> int<3> {
  // CHECK: %[[reg:.*]] = xor i24 %{{.*}}, -1
  // CHECK: ret i24 %[[reg]]
  !a
}

// CHECK-LABEL: define i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> int<4> {
  // CHECK: %[[reg:.*]] = xor i32 %{{.*}}, -1
  // CHECK: ret i32 %[[reg]]
  !a
}

// CHECK-LABEL: define i40 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> int<5> {
  // CHECK: %[[reg:.*]] = xor i40 %{{.*}}, -1
  // CHECK: ret i40 %[[reg]]
  !a
}

// CHECK-LABEL: define i48 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> int<6> {
  // CHECK: %[[reg:.*]] = xor i48 %{{.*}}, -1
  // CHECK: ret i48 %[[reg]]
  !a
}

// CHECK-LABEL: define i56 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> int<7> {
  // CHECK: %[[reg:.*]] = xor i56 %{{.*}}, -1
  // CHECK: ret i56 %[[reg]]
  !a
}

// CHECK-LABEL: define i64 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> int<8> {
  // CHECK: %[[reg:.*]] = xor i64 %{{.*}}, -1
  // CHECK: ret i64 %[[reg]]
  !a
}

// CHECK-LABEL: define void @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> int<9> {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i72 %[[reg_a]], -1
  // CHECK: store i72 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> int<10> {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i80 %[[reg_a]], -1
  // CHECK: store i80 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> int<11> {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i88 %[[reg_a]], -1
  // CHECK: store i88 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> int<12> {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i96 %[[reg_a]], -1
  // CHECK: store i96 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> int<13> {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i104 %[[reg_a]], -1
  // CHECK: store i104 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> int<14> {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i112 %[[reg_a]], -1
  // CHECK: store i112 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_15
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>) -> int<15> {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i120 %[[reg_a]], -1
  // CHECK: store i120 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>) -> int<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i128 %[[reg_a]], -1
  // CHECK: store i128 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> uint<1> {
  // CHECK: %[[reg:.*]] = xor i8 %{{.*}}, -1
  // CHECK: ret i8 %[[reg]]
  !a
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> uint<2> {
  // CHECK: %[[reg:.*]] = xor i16 %{{.*}}, -1
  // CHECK: ret i16 %[[reg]]
  !a
}

// CHECK-LABEL: define i24 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> uint<3> {
  // CHECK: %[[reg:.*]] = xor i24 %{{.*}}, -1
  // CHECK: ret i24 %[[reg]]
  !a
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> uint<4> {
  // CHECK: %[[reg:.*]] = xor i32 %{{.*}}, -1
  // CHECK: ret i32 %[[reg]]
  !a
}

// CHECK-LABEL: define i40 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> uint<5> {
  // CHECK: %[[reg:.*]] = xor i40 %{{.*}}, -1
  // CHECK: ret i40 %[[reg]]
  !a
}

// CHECK-LABEL: define i48 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> uint<6> {
  // CHECK: %[[reg:.*]] = xor i48 %{{.*}}, -1
  // CHECK: ret i48 %[[reg]]
  !a
}

// CHECK-LABEL: define i56 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> uint<7> {
  // CHECK: %[[reg:.*]] = xor i56 %{{.*}}, -1
  // CHECK: ret i56 %[[reg]]
  !a
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> uint<8> {
  // CHECK: %[[reg:.*]] = xor i64 %{{.*}}, -1
  // CHECK: ret i64 %[[reg]]
  !a
}

// CHECK-LABEL: define void @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> uint<9> {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i72 %[[reg_a]], -1
  // CHECK: store i72 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> uint<10> {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i80 %[[reg_a]], -1
  // CHECK: store i80 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> uint<11> {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i88 %[[reg_a]], -1
  // CHECK: store i88 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> uint<12> {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i96 %[[reg_a]], -1
  // CHECK: store i96 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> uint<13> {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i104 %[[reg_a]], -1
  // CHECK: store i104 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> uint<14> {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i112 %[[reg_a]], -1
  // CHECK: store i112 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>) -> uint<15> {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i120 %[[reg_a]], -1
  // CHECK: store i120 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = xor i128 %[[reg_a]], -1
  // CHECK: store i128 %[[reg_b]], ptr %{{.*}}, align 1
  // CHECK: ret void
  !a
}
