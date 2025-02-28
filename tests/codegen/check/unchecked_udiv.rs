use exint::uint;

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> uint<1> {
  // CHECK: %[[reg:.*]] = udiv i8 %{{.*}}, %{{.*}}
  // CHECK: ret i8 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<1>, 1>(a, b) }
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> uint<2> {
  // CHECK: %[[reg:.*]] = udiv i16 %{{.*}}, %{{.*}}
  // CHECK: ret i16 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<2>, 2>(a, b) }
}

// CHECK-LABEL: define i24 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> uint<3> {
  // CHECK: %[[reg:.*]] = udiv i24 %{{.*}}, %{{.*}}
  // CHECK: ret i24 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<3>, 3>(a, b) }
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> uint<4> {
  // CHECK: %[[reg:.*]] = udiv i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<4>, 4>(a, b) }
}

// CHECK-LABEL: define i40 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> uint<5> {
  // CHECK: %[[reg:.*]] = udiv i40 %{{.*}}, %{{.*}}
  // CHECK: ret i40 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<5>, 5>(a, b) }
}

// CHECK-LABEL: define i48 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> uint<6> {
  // CHECK: %[[reg:.*]] = udiv i48 %{{.*}}, %{{.*}}
  // CHECK: ret i48 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<6>, 6>(a, b) }
}

// CHECK-LABEL: define i56 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> uint<7> {
  // CHECK: %[[reg:.*]] = udiv i56 %{{.*}}, %{{.*}}
  // CHECK: ret i56 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<7>, 7>(a, b) }
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> uint<8> {
  // CHECK: %[[reg:.*]] = udiv i64 %{{.*}}, %{{.*}}
  // CHECK: ret i64 %[[reg]]
  unsafe { exint::backend::unchecked_udiv::<uint<8>, 8>(a, b) }
}

// CHECK-LABEL: define void @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> uint<9> {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i72 %[[reg_a]], %[[reg_b]]
  // CHECK: store i72 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<9>, 9>(a, b) }
}

// CHECK-LABEL: define void @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> uint<10> {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i80 %[[reg_a]], %[[reg_b]]
  // CHECK: store i80 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<10>, 10>(a, b) }
}

// CHECK-LABEL: define void @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> uint<11> {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i88 %[[reg_a]], %[[reg_b]]
  // CHECK: store i88 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<11>, 11>(a, b) }
}

// CHECK-LABEL: define void @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> uint<12> {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i96 %[[reg_a]], %[[reg_b]]
  // CHECK: store i96 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<12>, 12>(a, b) }
}

// CHECK-LABEL: define void @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> uint<13> {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i104 %[[reg_a]], %[[reg_b]]
  // CHECK: store i104 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<13>, 13>(a, b) }
}

// CHECK-LABEL: define void @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> uint<14> {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i112 %[[reg_a]], %[[reg_b]]
  // CHECK: store i112 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<14>, 14>(a, b) }
}

// CHECK-LABEL: define void @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> uint<15> {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i120 %[[reg_a]], %[[reg_b]]
  // CHECK: store i120 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<15>, 15>(a, b) }
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = udiv i128 %[[reg_a]], %[[reg_b]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<uint<16>, 16>(a, b) }
}
