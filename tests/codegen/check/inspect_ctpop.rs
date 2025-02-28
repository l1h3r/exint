use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i8 0, 9) i8 @llvm.ctpop.i8(i8 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i8 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i16 0, 17) i16 @llvm.ctpop.i16(i16 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i16 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 25) i32 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i24 0, 25) i24 @llvm.ctpop.i24(i24 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i24 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> u32 {
  // CHECK: %[[reg:.*]] = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i40 0, 41) i40 @llvm.ctpop.i40(i40 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i40 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 49) i32 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i48 0, 49) i48 @llvm.ctpop.i48(i48 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i48 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i56 0, 57) i56 @llvm.ctpop.i56(i56 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i56 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i64 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 73) i32 @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i72 0, 73) i72 @llvm.ctpop.i72(i72 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i72 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 81) i32 @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i80 0, 81) i80 @llvm.ctpop.i80(i80 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i80 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 89) i32 @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i88 0, 89) i88 @llvm.ctpop.i88(i88 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i88 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 97) i32 @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i96 0, 97) i96 @llvm.ctpop.i96(i96 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i96 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i104 0, 105) i104 @llvm.ctpop.i104(i104 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i104 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 113) i32 @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i112 0, 113) i112 @llvm.ctpop.i112(i112 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i112 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 121) i32 @int_15
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i120 0, 121) i120 @llvm.ctpop.i120(i120 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i120 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i128 0, 129) i128 @llvm.ctpop.i128(i128 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i128 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i8 0, 9) i8 @llvm.ctpop.i8(i8 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i8 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i16 0, 17) i16 @llvm.ctpop.i16(i16 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i16 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 25) i32 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i24 0, 25) i24 @llvm.ctpop.i24(i24 %{{.*}})
  // CHECK: %[[reg_b:.*]] = zext nneg i24 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> u32 {
  // CHECK: %[[reg:.*]] = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %{{.*}})
  // CHECK: ret i32 %[[reg]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i40 0, 41) i40 @llvm.ctpop.i40(i40 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i40 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 49) i32 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i48 0, 49) i48 @llvm.ctpop.i48(i48 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i48 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i56 0, 57) i56 @llvm.ctpop.i56(i56 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i56 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %{{.*}})
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i64 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 73) i32 @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i72 0, 73) i72 @llvm.ctpop.i72(i72 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i72 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 81) i32 @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i80 0, 81) i80 @llvm.ctpop.i80(i80 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i80 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 89) i32 @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i88 0, 89) i88 @llvm.ctpop.i88(i88 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i88 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 97) i32 @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i96 0, 97) i96 @llvm.ctpop.i96(i96 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i96 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i104 0, 105) i104 @llvm.ctpop.i104(i104 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i104 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 113) i32 @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i112 0, 113) i112 @llvm.ctpop.i112(i112 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i112 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 121) i32 @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i120 0, 121) i120 @llvm.ctpop.i120(i120 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i120 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i128 0, 129) i128 @llvm.ctpop.i128(i128 %[[reg_a]])
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i128 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  a.count_ones()
}
