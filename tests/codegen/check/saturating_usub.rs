use exint::uint;

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> uint<1> {
  // CHECK: %[[reg:.*]] = tail call i8 @llvm.usub.sat.i8(i8 %{{.*}}, i8 %{{.*}})
  // CHECK: ret i8 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> uint<2> {
  // CHECK: %[[reg:.*]] = tail call i16 @llvm.usub.sat.i16(i16 %{{.*}}, i16 %{{.*}})
  // CHECK: ret i16 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i24 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> uint<3> {
  // CHECK: %[[reg:.*]] = tail call i24 @llvm.usub.sat.i24(i24 %{{.*}}, i24 %{{.*}})
  // CHECK: ret i24 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> uint<4> {
  // CHECK: %[[reg:.*]] = tail call i32 @llvm.usub.sat.i32(i32 %{{.*}}, i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i40 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> uint<5> {
  // CHECK: %[[reg:.*]] = tail call i40 @llvm.usub.sat.i40(i40 %{{.*}}, i40 %{{.*}})
  // CHECK: ret i40 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i48 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> uint<6> {
  // CHECK: %[[reg:.*]] = tail call i48 @llvm.usub.sat.i48(i48 %{{.*}}, i48 %{{.*}})
  // CHECK: ret i48 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i56 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> uint<7> {
  // CHECK: %[[reg:.*]] = tail call i56 @llvm.usub.sat.i56(i56 %{{.*}}, i56 %{{.*}})
  // CHECK: ret i56 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> uint<8> {
  // CHECK: %[[reg:.*]] = tail call i64 @llvm.usub.sat.i64(i64 %{{.*}}, i64 %{{.*}})
  // CHECK: ret i64 %[[reg]]
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> uint<9> {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i72 @llvm.usub.sat.i72(i72 %[[reg_a]], i72 %[[reg_b]])
  // CHECK: store i72 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> uint<10> {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i80 @llvm.usub.sat.i80(i80 %[[reg_a]], i80 %[[reg_b]])
  // CHECK: store i80 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> uint<11> {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i88 @llvm.usub.sat.i88(i88 %[[reg_a]], i88 %[[reg_b]])
  // CHECK: store i88 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> uint<12> {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i96 @llvm.usub.sat.i96(i96 %[[reg_a]], i96 %[[reg_b]])
  // CHECK: store i96 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> uint<13> {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i104 @llvm.usub.sat.i104(i104 %[[reg_a]], i104 %[[reg_b]])
  // CHECK: store i104 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> uint<14> {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i112 @llvm.usub.sat.i112(i112 %[[reg_a]], i112 %[[reg_b]])
  // CHECK: store i112 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> uint<15> {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i120 @llvm.usub.sat.i120(i120 %[[reg_a]], i120 %[[reg_b]])
  // CHECK: store i120 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = tail call i128 @llvm.usub.sat.i128(i128 %[[reg_a]], i128 %[[reg_b]])
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  a.saturating_sub(b)
}
