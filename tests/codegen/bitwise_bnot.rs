#![feature(core_intrinsics)]
#![allow(internal_features)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @bitwise_bnot_int_1
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_1(a: int<1>) -> int<1> {
// CHECK: xor i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_2
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_2(a: int<2>) -> int<2> {
// CHECK: xor i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_3
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_3(a: int<3>) -> int<3> {
// CHECK: xor i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i24 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_4
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_4(a: int<4>) -> int<4> {
// CHECK: xor i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_5
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_5(a: int<5>) -> int<5> {
// CHECK: xor i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i40 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_6
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_6(a: int<6>) -> int<6> {
// CHECK: xor i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_7
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_7(a: int<7>) -> int<7> {
// CHECK: xor i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i56 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_8
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_8(a: int<8>) -> int<8> {
// CHECK: xor i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_9
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_9(a: int<9>) -> int<9> {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_10
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_10(a: int<10>) -> int<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_11
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_11(a: int<11>) -> int<11> {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_12
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_12(a: int<12>) -> int<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_13
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_13(a: int<13>) -> int<13> {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_14
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_14(a: int<14>) -> int<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_15
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_15(a: int<15>) -> int<15> {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_int_16
#[unsafe(no_mangle)]
pub fn bitwise_bnot_int_16(a: int<16>) -> int<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_1
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_1(a: uint<1>) -> uint<1> {
// CHECK: xor i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_2
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_2(a: uint<2>) -> uint<2> {
// CHECK: xor i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_3
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_3(a: uint<3>) -> uint<3> {
// CHECK: xor i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i24 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_4
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_4(a: uint<4>) -> uint<4> {
// CHECK: xor i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_5
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_5(a: uint<5>) -> uint<5> {
// CHECK: xor i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i40 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_6
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_6(a: uint<6>) -> uint<6> {
// CHECK: xor i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_7
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_7(a: uint<7>) -> uint<7> {
// CHECK: xor i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i56 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_8
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_8(a: uint<8>) -> uint<8> {
// CHECK: xor i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_9
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_9(a: uint<9>) -> uint<9> {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_10
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_10(a: uint<10>) -> uint<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_11
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_11(a: uint<11>) -> uint<11> {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_12
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_12(a: uint<12>) -> uint<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_13
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_13(a: uint<13>) -> uint<13> {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_14
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_14(a: uint<14>) -> uint<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_15
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_15(a: uint<15>) -> uint<15> {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}

// CHECK-LABEL: @bitwise_bnot_uint_16
#[unsafe(no_mangle)]
pub fn bitwise_bnot_uint_16(a: uint<16>) -> uint<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: xor i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], -1
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  !a
}
