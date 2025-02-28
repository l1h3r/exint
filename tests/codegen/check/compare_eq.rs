use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef zeroext i1 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>, b: int<1>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i8 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>, b: int<2>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i16 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>, b: int<3>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i24 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>, b: int<4>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i32 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>, b: int<5>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i40 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>, b: int<6>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i48 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>, b: int<7>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i56 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>, b: int<8>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i64 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>, b: int<9>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i72 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>, b: int<10>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i80 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>, b: int<11>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i88 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>, b: int<12>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i96 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>, b: int<13>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i104 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>, b: int<14>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i112 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_15
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>, b: int<15>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i120 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>, b: int<16>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i128 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i8 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i16 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i24 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i32 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i40 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i48 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i56 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> bool {
  // CHECK: %[[reg:.*]] = icmp eq i64 %{{.*}}, %{{.*}}
  // CHECK: ret i1 %[[reg]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i72 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i80 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i88 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i96 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i104 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i112 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i120 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}

// CHECK-LABEL: define noundef zeroext i1 @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> bool {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = icmp eq i128 %[[reg_a]], %[[reg_b]]
  // CHECK: ret i1 %[[reg_c]]
  a == b
}
