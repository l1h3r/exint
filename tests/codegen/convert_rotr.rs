#![feature(core_intrinsics)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @convert_rotr_int_1
#[no_mangle]
pub fn convert_rotr_int_1(a: int<1>, b: u32) -> int<1> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: @llvm.fshr.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_2
#[no_mangle]
pub fn convert_rotr_int_2(a: int<2>, b: u32) -> int<2> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i16
// CHECK: @llvm.fshr.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_3
#[no_mangle]
pub fn convert_rotr_int_3(a: int<3>, b: u32) -> int<3> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i24
// CHECK: @llvm.fshr.i24(i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i24 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i24 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_4
#[no_mangle]
pub fn convert_rotr_int_4(a: int<4>, b: u32) -> int<4> {
// CHECK: @llvm.fshr.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_5
#[no_mangle]
pub fn convert_rotr_int_5(a: int<5>, b: u32) -> int<5> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i40
// CHECK: @llvm.fshr.i40(i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i40 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i40 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_6
#[no_mangle]
pub fn convert_rotr_int_6(a: int<6>, b: u32) -> int<6> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i48
// CHECK: @llvm.fshr.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i48 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_7
#[no_mangle]
pub fn convert_rotr_int_7(a: int<7>, b: u32) -> int<7> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i56
// CHECK: @llvm.fshr.i56(i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i56 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i56 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_8
#[no_mangle]
pub fn convert_rotr_int_8(a: int<8>, b: u32) -> int<8> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i64
// CHECK: @llvm.fshr.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_9
#[no_mangle]
pub fn convert_rotr_int_9(a: int<9>, b: u32) -> int<9> {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i72
// CHECK: @llvm.fshr.i72(i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i72 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_10
#[no_mangle]
pub fn convert_rotr_int_10(a: int<10>, b: u32) -> int<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i80
// CHECK: @llvm.fshr.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i80 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_11
#[no_mangle]
pub fn convert_rotr_int_11(a: int<11>, b: u32) -> int<11> {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i88
// CHECK: @llvm.fshr.i88(i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i88 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_12
#[no_mangle]
pub fn convert_rotr_int_12(a: int<12>, b: u32) -> int<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i96
// CHECK: @llvm.fshr.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i96 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_13
#[no_mangle]
pub fn convert_rotr_int_13(a: int<13>, b: u32) -> int<13> {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i104
// CHECK: @llvm.fshr.i104(i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i104 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_14
#[no_mangle]
pub fn convert_rotr_int_14(a: int<14>, b: u32) -> int<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i112
// CHECK: @llvm.fshr.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i112 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_15
#[no_mangle]
pub fn convert_rotr_int_15(a: int<15>, b: u32) -> int<15> {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i120
// CHECK: @llvm.fshr.i120(i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i120 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_16
#[no_mangle]
pub fn convert_rotr_int_16(a: int<16>, b: u32) -> int<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i128
// CHECK: @llvm.fshr.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_1
#[no_mangle]
pub fn convert_rotr_uint_1(a: uint<1>, b: u32) -> uint<1> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: @llvm.fshr.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_2
#[no_mangle]
pub fn convert_rotr_uint_2(a: uint<2>, b: u32) -> uint<2> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i16
// CHECK: @llvm.fshr.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_3
#[no_mangle]
pub fn convert_rotr_uint_3(a: uint<3>, b: u32) -> uint<3> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i24
// CHECK: @llvm.fshr.i24(i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i24 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i24 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_4
#[no_mangle]
pub fn convert_rotr_uint_4(a: uint<4>, b: u32) -> uint<4> {
// CHECK: @llvm.fshr.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_5
#[no_mangle]
pub fn convert_rotr_uint_5(a: uint<5>, b: u32) -> uint<5> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i40
// CHECK: @llvm.fshr.i40(i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i40 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i40 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_6
#[no_mangle]
pub fn convert_rotr_uint_6(a: uint<6>, b: u32) -> uint<6> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i48
// CHECK: @llvm.fshr.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i48 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_7
#[no_mangle]
pub fn convert_rotr_uint_7(a: uint<7>, b: u32) -> uint<7> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i56
// CHECK: @llvm.fshr.i56(i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i56 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i56 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_8
#[no_mangle]
pub fn convert_rotr_uint_8(a: uint<8>, b: u32) -> uint<8> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i64
// CHECK: @llvm.fshr.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_9
#[no_mangle]
pub fn convert_rotr_uint_9(a: uint<9>, b: u32) -> uint<9> {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i72
// CHECK: @llvm.fshr.i72(i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i72 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_10
#[no_mangle]
pub fn convert_rotr_uint_10(a: uint<10>, b: u32) -> uint<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i80
// CHECK: @llvm.fshr.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i80 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_11
#[no_mangle]
pub fn convert_rotr_uint_11(a: uint<11>, b: u32) -> uint<11> {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i88
// CHECK: @llvm.fshr.i88(i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i88 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_12
#[no_mangle]
pub fn convert_rotr_uint_12(a: uint<12>, b: u32) -> uint<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i96
// CHECK: @llvm.fshr.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i96 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_13
#[no_mangle]
pub fn convert_rotr_uint_13(a: uint<13>, b: u32) -> uint<13> {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i104
// CHECK: @llvm.fshr.i104(i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i104 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_14
#[no_mangle]
pub fn convert_rotr_uint_14(a: uint<14>, b: u32) -> uint<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i112
// CHECK: @llvm.fshr.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i112 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_15
#[no_mangle]
pub fn convert_rotr_uint_15(a: uint<15>, b: u32) -> uint<15> {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i120
// CHECK: @llvm.fshr.i120(i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i120 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_16
#[no_mangle]
pub fn convert_rotr_uint_16(a: uint<16>, b: u32) -> uint<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i128
// CHECK: @llvm.fshr.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}
