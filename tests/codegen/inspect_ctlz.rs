#![feature(core_intrinsics)]
#![allow(internal_features)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @inspect_ctlz_int_1
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_1(a: int<1>) -> u32 {
// CHECK: @llvm.ctlz.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i8 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_2
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_2(a: int<2>) -> u32 {
// CHECK: @llvm.ctlz.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i16 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_3
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_3(a: int<3>) -> u32 {
// CHECK: @llvm.ctlz.i24(i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i24 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_4
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_4(a: int<4>) -> u32 {
// CHECK: @llvm.ctlz.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_5
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_5(a: int<5>) -> u32 {
// CHECK: @llvm.ctlz.i40(i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i40 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_6
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_6(a: int<6>) -> u32 {
// CHECK: @llvm.ctlz.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i48 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_7
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_7(a: int<7>) -> u32 {
// CHECK: @llvm.ctlz.i56(i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i56 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_8
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_8(a: int<8>) -> u32 {
// CHECK: @llvm.ctlz.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i64 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_9
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_9(a: int<9>) -> u32 {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i72(i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i72 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_10
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_10(a: int<10>) -> u32 {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i80 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_11
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_11(a: int<11>) -> u32 {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i88(i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i88 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_12
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_12(a: int<12>) -> u32 {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i96 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_13
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_13(a: int<13>) -> u32 {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i104(i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i104 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_14
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_14(a: int<14>) -> u32 {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i112 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_15
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_15(a: int<15>) -> u32 {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i120(i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i120 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_int_16
#[unsafe(no_mangle)]
pub fn inspect_ctlz_int_16(a: int<16>) -> u32 {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i128 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_1
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_1(a: uint<1>) -> u32 {
// CHECK: @llvm.ctlz.i8(i8 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i8 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_2
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_2(a: uint<2>) -> u32 {
// CHECK: @llvm.ctlz.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i16 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_3
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_3(a: uint<3>) -> u32 {
// CHECK: @llvm.ctlz.i24(i24 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: zext i24 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_4
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_4(a: uint<4>) -> u32 {
// CHECK: @llvm.ctlz.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_5
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_5(a: uint<5>) -> u32 {
// CHECK: @llvm.ctlz.i40(i40 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i40 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_6
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_6(a: uint<6>) -> u32 {
// CHECK: @llvm.ctlz.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i48 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_7
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_7(a: uint<7>) -> u32 {
// CHECK: @llvm.ctlz.i56(i56 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i56 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_8
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_8(a: uint<8>) -> u32 {
// CHECK: @llvm.ctlz.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i64 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_9
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_9(a: uint<9>) -> u32 {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i72(i72 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i72 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_10
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_10(a: uint<10>) -> u32 {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i80 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_11
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_11(a: uint<11>) -> u32 {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i88(i88 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i88 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_12
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_12(a: uint<12>) -> u32 {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i96 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_13
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_13(a: uint<13>) -> u32 {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i104(i104 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i104 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_14
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_14(a: uint<14>) -> u32 {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i112 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_15
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_15(a: uint<15>) -> u32 {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i120(i120 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i120 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}

// CHECK-LABEL: @inspect_ctlz_uint_16
#[unsafe(no_mangle)]
pub fn inspect_ctlz_uint_16(a: uint<16>) -> u32 {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.ctlz.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], i1 false)
// CHECK: trunc i128 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i32
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.leading_zeros()
}
