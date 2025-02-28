use exint::int;

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i8(i8 %{{.*}}, i8 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i16(i16 %{{.*}}, i16 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i24(i24 %{{.*}}, i24 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i32(i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i40(i40 %{{.*}}, i40 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i48(i48 %{{.*}}, i48 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i56(i56 %{{.*}}, i56 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.scmp.i8.i64(i64 %{{.*}}, i64 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i72(i72 %[[reg_a]], i72 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i80(i80 %[[reg_a]], i80 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i88(i88 %[[reg_a]], i88 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i96(i96 %[[reg_a]], i96 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i104(i104 %[[reg_a]], i104 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i112(i112 %[[reg_a]], i112 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_15
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i120(i120 %[[reg_a]], i120 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> ::core::cmp::Ordering {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i8 @llvm.scmp.i8.i128(i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: ret i8 %[[reg_c]]
  a.cmp(&b)
}
