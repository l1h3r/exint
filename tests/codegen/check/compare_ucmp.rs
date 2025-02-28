use exint::uint;

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i8(i8 %{{.*}}, i8 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i16(i16 %{{.*}}, i16 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i24(i24 %{{.*}}, i24 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i32(i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i40(i40 %{{.*}}, i40 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i48(i48 %{{.*}}, i48 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i56(i56 %{{.*}}, i56 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.ucmp.i8.i64(i64 %{{.*}}, i64 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i72(i72 %[[reg_a]], i72 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i80(i80 %[[reg_a]], i80 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i88(i88 %[[reg_a]], i88 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i96(i96 %[[reg_a]], i96 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i104(i104 %[[reg_a]], i104 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i112(i112 %[[reg_a]], i112 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i120(i120 %[[reg_a]], i120 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.ucmp.i8.i128(i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}
