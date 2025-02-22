#![feature(core_intrinsics)]
use exint_integer::int;
use exint_integer::uint;

// CHECK-LABEL: @convert_swap8_int_2
#[no_mangle]
pub fn convert_swap8_int_2(a: int<2>) -> int<2> {
// CHECK: @llvm.bswap.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_4
#[no_mangle]
pub fn convert_swap8_int_4(a: int<4>) -> int<4> {
// CHECK: @llvm.bswap.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_6
#[no_mangle]
pub fn convert_swap8_int_6(a: int<6>) -> int<6> {
// CHECK: @llvm.bswap.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_8
#[no_mangle]
pub fn convert_swap8_int_8(a: int<8>) -> int<8> {
// CHECK: @llvm.bswap.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_10
#[no_mangle]
pub fn convert_swap8_int_10(a: int<10>) -> int<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_12
#[no_mangle]
pub fn convert_swap8_int_12(a: int<12>) -> int<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_14
#[no_mangle]
pub fn convert_swap8_int_14(a: int<14>) -> int<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_int_16
#[no_mangle]
pub fn convert_swap8_int_16(a: int<16>) -> int<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_2
#[no_mangle]
pub fn convert_swap8_uint_2(a: uint<2>) -> uint<2> {
// CHECK: @llvm.bswap.i16(i16 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i16 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_4
#[no_mangle]
pub fn convert_swap8_uint_4(a: uint<4>) -> uint<4> {
// CHECK: @llvm.bswap.i32(i32 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i32 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_6
#[no_mangle]
pub fn convert_swap8_uint_6(a: uint<6>) -> uint<6> {
// CHECK: @llvm.bswap.i48(i48 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i48 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_8
#[no_mangle]
pub fn convert_swap8_uint_8(a: uint<8>) -> uint<8> {
// CHECK: @llvm.bswap.i64(i64 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: ret i64 [[REGISTER:%[-a-zA-Z0-9$._]+]]
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_10
#[no_mangle]
pub fn convert_swap8_uint_10(a: uint<10>) -> uint<10> {
// CHECK: load i80, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i80(i80 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i80 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_12
#[no_mangle]
pub fn convert_swap8_uint_12(a: uint<12>) -> uint<12> {
// CHECK: load i96, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i96(i96 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i96 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_14
#[no_mangle]
pub fn convert_swap8_uint_14(a: uint<14>) -> uint<14> {
// CHECK: load i112, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i112(i112 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i112 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}

// CHECK-LABEL: @convert_swap8_uint_16
#[no_mangle]
pub fn convert_swap8_uint_16(a: uint<16>) -> uint<16> {
// CHECK: load i128, ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: @llvm.bswap.i128(i128 [[REGISTER:%[-a-zA-Z0-9$._]+]])
// CHECK: store i128 [[REGISTER:%[-a-zA-Z0-9$._]+]], ptr [[REGISTER:%[-a-zA-Z0-9$._]+]]
// CHECK: ret void
  a.swap_bytes()
}
