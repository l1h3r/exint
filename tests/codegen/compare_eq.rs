#![feature(core_intrinsics)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @compare_eq_int_1
#[no_mangle]
pub fn compare_eq_int_1(a: int<1>, b: int<1>) -> bool {
// CHECK: icmp eq i8
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_2
#[no_mangle]
pub fn compare_eq_int_2(a: int<2>, b: int<2>) -> bool {
// CHECK: icmp eq i16
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_3
#[no_mangle]
pub fn compare_eq_int_3(a: int<3>, b: int<3>) -> bool {
// CHECK: icmp eq i24
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_4
#[no_mangle]
pub fn compare_eq_int_4(a: int<4>, b: int<4>) -> bool {
// CHECK: icmp eq i32
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_5
#[no_mangle]
pub fn compare_eq_int_5(a: int<5>, b: int<5>) -> bool {
// CHECK: icmp eq i40
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_6
#[no_mangle]
pub fn compare_eq_int_6(a: int<6>, b: int<6>) -> bool {
// CHECK: icmp eq i48
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_7
#[no_mangle]
pub fn compare_eq_int_7(a: int<7>, b: int<7>) -> bool {
// CHECK: icmp eq i56
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_8
#[no_mangle]
pub fn compare_eq_int_8(a: int<8>, b: int<8>) -> bool {
// CHECK: icmp eq i64
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_9
#[no_mangle]
pub fn compare_eq_int_9(a: int<9>, b: int<9>) -> bool {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i72
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_10
#[no_mangle]
pub fn compare_eq_int_10(a: int<10>, b: int<10>) -> bool {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i80
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_11
#[no_mangle]
pub fn compare_eq_int_11(a: int<11>, b: int<11>) -> bool {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i88
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_12
#[no_mangle]
pub fn compare_eq_int_12(a: int<12>, b: int<12>) -> bool {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i96
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_13
#[no_mangle]
pub fn compare_eq_int_13(a: int<13>, b: int<13>) -> bool {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i104
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_14
#[no_mangle]
pub fn compare_eq_int_14(a: int<14>, b: int<14>) -> bool {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i112
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_15
#[no_mangle]
pub fn compare_eq_int_15(a: int<15>, b: int<15>) -> bool {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i120
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_int_16
#[no_mangle]
pub fn compare_eq_int_16(a: int<16>, b: int<16>) -> bool {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i128
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_1
#[no_mangle]
pub fn compare_eq_uint_1(a: uint<1>, b: uint<1>) -> bool {
// CHECK: icmp eq i8
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_2
#[no_mangle]
pub fn compare_eq_uint_2(a: uint<2>, b: uint<2>) -> bool {
// CHECK: icmp eq i16
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_3
#[no_mangle]
pub fn compare_eq_uint_3(a: uint<3>, b: uint<3>) -> bool {
// CHECK: icmp eq i24
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_4
#[no_mangle]
pub fn compare_eq_uint_4(a: uint<4>, b: uint<4>) -> bool {
// CHECK: icmp eq i32
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_5
#[no_mangle]
pub fn compare_eq_uint_5(a: uint<5>, b: uint<5>) -> bool {
// CHECK: icmp eq i40
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_6
#[no_mangle]
pub fn compare_eq_uint_6(a: uint<6>, b: uint<6>) -> bool {
// CHECK: icmp eq i48
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_7
#[no_mangle]
pub fn compare_eq_uint_7(a: uint<7>, b: uint<7>) -> bool {
// CHECK: icmp eq i56
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_8
#[no_mangle]
pub fn compare_eq_uint_8(a: uint<8>, b: uint<8>) -> bool {
// CHECK: icmp eq i64
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_9
#[no_mangle]
pub fn compare_eq_uint_9(a: uint<9>, b: uint<9>) -> bool {
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i72, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i72
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_10
#[no_mangle]
pub fn compare_eq_uint_10(a: uint<10>, b: uint<10>) -> bool {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i80
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_11
#[no_mangle]
pub fn compare_eq_uint_11(a: uint<11>, b: uint<11>) -> bool {
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i88, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i88
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_12
#[no_mangle]
pub fn compare_eq_uint_12(a: uint<12>, b: uint<12>) -> bool {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i96
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_13
#[no_mangle]
pub fn compare_eq_uint_13(a: uint<13>, b: uint<13>) -> bool {
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i104, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i104
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_14
#[no_mangle]
pub fn compare_eq_uint_14(a: uint<14>, b: uint<14>) -> bool {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i112
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_15
#[no_mangle]
pub fn compare_eq_uint_15(a: uint<15>, b: uint<15>) -> bool {
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i120, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i120
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}

// CHECK-LABEL: @compare_eq_uint_16
#[no_mangle]
pub fn compare_eq_uint_16(a: uint<16>, b: uint<16>) -> bool {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: icmp eq i128
// CHECK: ret i1 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.eq(&b)
}
