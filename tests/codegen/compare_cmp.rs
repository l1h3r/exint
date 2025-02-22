#![feature(core_intrinsics)]
#![allow(internal_features)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @compare_cmp_int_1
#[unsafe(no_mangle)]
pub fn compare_cmp_int_1(a: int<1>, b: int<1>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i8
// CHECK: icmp ne i8
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_2
#[unsafe(no_mangle)]
pub fn compare_cmp_int_2(a: int<2>, b: int<2>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i16
// CHECK: icmp ne i16
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_3
#[unsafe(no_mangle)]
pub fn compare_cmp_int_3(a: int<3>, b: int<3>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i24
// CHECK: icmp ne i24
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_4
#[unsafe(no_mangle)]
pub fn compare_cmp_int_4(a: int<4>, b: int<4>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i32
// CHECK: icmp ne i32
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_5
#[unsafe(no_mangle)]
pub fn compare_cmp_int_5(a: int<5>, b: int<5>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i40
// CHECK: icmp ne i40
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_6
#[unsafe(no_mangle)]
pub fn compare_cmp_int_6(a: int<6>, b: int<6>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i48
// CHECK: icmp ne i48
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_7
#[unsafe(no_mangle)]
pub fn compare_cmp_int_7(a: int<7>, b: int<7>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i56
// CHECK: icmp ne i56
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_8
#[unsafe(no_mangle)]
pub fn compare_cmp_int_8(a: int<8>, b: int<8>) -> ::core::cmp::Ordering {
// CHECK: icmp slt i64
// CHECK: icmp ne i64
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_9
#[unsafe(no_mangle)]
pub fn compare_cmp_int_9(a: int<9>, b: int<9>) -> ::core::cmp::Ordering {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i72
// CHECK: icmp ne i72
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_10
#[unsafe(no_mangle)]
pub fn compare_cmp_int_10(a: int<10>, b: int<10>) -> ::core::cmp::Ordering {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i80
// CHECK: icmp ne i80
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_11
#[unsafe(no_mangle)]
pub fn compare_cmp_int_11(a: int<11>, b: int<11>) -> ::core::cmp::Ordering {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i88
// CHECK: icmp ne i88
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_12
#[unsafe(no_mangle)]
pub fn compare_cmp_int_12(a: int<12>, b: int<12>) -> ::core::cmp::Ordering {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i96
// CHECK: icmp ne i96
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_13
#[unsafe(no_mangle)]
pub fn compare_cmp_int_13(a: int<13>, b: int<13>) -> ::core::cmp::Ordering {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i104
// CHECK: icmp ne i104
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_14
#[unsafe(no_mangle)]
pub fn compare_cmp_int_14(a: int<14>, b: int<14>) -> ::core::cmp::Ordering {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i112
// CHECK: icmp ne i112
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_15
#[unsafe(no_mangle)]
pub fn compare_cmp_int_15(a: int<15>, b: int<15>) -> ::core::cmp::Ordering {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i120
// CHECK: icmp ne i120
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_int_16
#[unsafe(no_mangle)]
pub fn compare_cmp_int_16(a: int<16>, b: int<16>) -> ::core::cmp::Ordering {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp slt i128
// CHECK: icmp ne i128
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_1
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_1(a: uint<1>, b: uint<1>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i8
// CHECK: icmp ne i8
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_2
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_2(a: uint<2>, b: uint<2>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i16
// CHECK: icmp ne i16
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_3
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_3(a: uint<3>, b: uint<3>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i24
// CHECK: icmp ne i24
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_4
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_4(a: uint<4>, b: uint<4>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i32
// CHECK: icmp ne i32
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_5
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_5(a: uint<5>, b: uint<5>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i40
// CHECK: icmp ne i40
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_6
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_6(a: uint<6>, b: uint<6>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i48
// CHECK: icmp ne i48
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_7
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_7(a: uint<7>, b: uint<7>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i56
// CHECK: icmp ne i56
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_8
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_8(a: uint<8>, b: uint<8>) -> ::core::cmp::Ordering {
// CHECK: icmp ult i64
// CHECK: icmp ne i64
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_9
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_9(a: uint<9>, b: uint<9>) -> ::core::cmp::Ordering {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i72
// CHECK: icmp ne i72
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_10
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_10(a: uint<10>, b: uint<10>) -> ::core::cmp::Ordering {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i80
// CHECK: icmp ne i80
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_11
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_11(a: uint<11>, b: uint<11>) -> ::core::cmp::Ordering {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i88
// CHECK: icmp ne i88
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_12
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_12(a: uint<12>, b: uint<12>) -> ::core::cmp::Ordering {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i96
// CHECK: icmp ne i96
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_13
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_13(a: uint<13>, b: uint<13>) -> ::core::cmp::Ordering {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i104
// CHECK: icmp ne i104
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_14
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_14(a: uint<14>, b: uint<14>) -> ::core::cmp::Ordering {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i112
// CHECK: icmp ne i112
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_15
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_15(a: uint<15>, b: uint<15>) -> ::core::cmp::Ordering {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i120
// CHECK: icmp ne i120
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}

// CHECK-LABEL: @compare_cmp_uint_16
#[unsafe(no_mangle)]
pub fn compare_cmp_uint_16(a: uint<16>, b: uint<16>) -> ::core::cmp::Ordering {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp ult i128
// CHECK: icmp ne i128
// CHECK: zext i1 [[REGISTER:%[-a-zA-Z0-9$._]+]] to i8
// CHECK: select i1 [[REGISTER:%[-a-zA-Z0-9$._]+]], i8 -1, i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret i8 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.cmp(&b)
}
