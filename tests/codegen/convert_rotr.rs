#![feature(core_intrinsics)]
#![allow(internal_features)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @convert_rotr_int_1
#[unsafe(no_mangle)]
pub fn convert_rotr_int_1(a: int<1>, b: u32) -> int<1> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: @llvm.fshr.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_2
#[unsafe(no_mangle)]
pub fn convert_rotr_int_2(a: int<2>, b: u32) -> int<2> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i16
// CHECK: @llvm.fshr.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_4
#[unsafe(no_mangle)]
pub fn convert_rotr_int_4(a: int<4>, b: u32) -> int<4> {
// CHECK: @llvm.fshr.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_8
#[unsafe(no_mangle)]
pub fn convert_rotr_int_8(a: int<8>, b: u32) -> int<8> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i64
// CHECK: @llvm.fshr.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_int_16
#[unsafe(no_mangle)]
pub fn convert_rotr_int_16(a: int<16>, b: u32) -> int<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i128
// CHECK: @llvm.fshr.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_1
#[unsafe(no_mangle)]
pub fn convert_rotr_uint_1(a: uint<1>, b: u32) -> uint<1> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: @llvm.fshr.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_2
#[unsafe(no_mangle)]
pub fn convert_rotr_uint_2(a: uint<2>, b: u32) -> uint<2> {
// CHECK: trunc i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i16
// CHECK: @llvm.fshr.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_4
#[unsafe(no_mangle)]
pub fn convert_rotr_uint_4(a: uint<4>, b: u32) -> uint<4> {
// CHECK: @llvm.fshr.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_8
#[unsafe(no_mangle)]
pub fn convert_rotr_uint_8(a: uint<8>, b: u32) -> uint<8> {
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i64
// CHECK: @llvm.fshr.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.rotate_right(b)
}

// CHECK-LABEL: @convert_rotr_uint_16
#[unsafe(no_mangle)]
pub fn convert_rotr_uint_16(a: uint<16>, b: u32) -> uint<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: zext i32 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i128
// CHECK: @llvm.fshr.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.rotate_right(b)
}
