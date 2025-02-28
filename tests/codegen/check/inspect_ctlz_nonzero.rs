use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @int_1
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i8 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<1>, 1>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 16) i32 @int_2
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i16 0, 17) i16 @llvm.ctlz.i16(i16 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i16 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<2>, 2>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 24) i32 @int_3
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i24 0, 25) i24 @llvm.ctlz.i24(i24 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i24 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<3>, 3>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 32) i32 @int_4
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> u32 {
  // CHECK: %[[reg:.*]] = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %{{.*}}, i1 true)
  // CHECK: ret i32 %[[reg]]
  unsafe { exint::backend::ctlz_nonzero::<int<4>, 4>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 40) i32 @int_5
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i40 0, 41) i40 @llvm.ctlz.i40(i40 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i40 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<5>, 5>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 48) i32 @int_6
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i48 0, 49) i48 @llvm.ctlz.i48(i48 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i48 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<6>, 6>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 56) i32 @int_7
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i56 0, 57) i56 @llvm.ctlz.i56(i56 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i56 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<7>, 7>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @int_8
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i64 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<int<8>, 8>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 72) i32 @int_9
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i72 0, 73) i72 @llvm.ctlz.i72(i72 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i72 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<9>, 9>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 80) i32 @int_10
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i80 0, 81) i80 @llvm.ctlz.i80(i80 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i80 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<10>, 10>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 88) i32 @int_11
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i88 0, 89) i88 @llvm.ctlz.i88(i88 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i88 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<11>, 11>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 96) i32 @int_12
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i96 0, 97) i96 @llvm.ctlz.i96(i96 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i96 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<12>, 12>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 104) i32 @int_13
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i104 0, 105) i104 @llvm.ctlz.i104(i104 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i104 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<13>, 13>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 112) i32 @int_14
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i112 0, 113) i112 @llvm.ctlz.i112(i112 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i112 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<14>, 14>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 120) i32 @int_15
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i120 0, 121) i120 @llvm.ctlz.i120(i120 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i120 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<15>, 15>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 128) i32 @int_16
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i128 0, 129) i128 @llvm.ctlz.i128(i128 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i128 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<int<16>, 16>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i8 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<1>, 1>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 16) i32 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i16 0, 17) i16 @llvm.ctlz.i16(i16 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i16 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<2>, 2>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 24) i32 @uint_3
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i24 0, 25) i24 @llvm.ctlz.i24(i24 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = zext nneg i24 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<3>, 3>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 32) i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> u32 {
  // CHECK: %[[reg:.*]] = tail call range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %{{.*}}, i1 true)
  // CHECK: ret i32 %[[reg]]
  unsafe { exint::backend::ctlz_nonzero::<uint<4>, 4>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 40) i32 @uint_5
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i40 0, 41) i40 @llvm.ctlz.i40(i40 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i40 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<5>, 5>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 48) i32 @uint_6
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i48 0, 49) i48 @llvm.ctlz.i48(i48 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i48 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<6>, 6>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 56) i32 @uint_7
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i56 0, 57) i56 @llvm.ctlz.i56(i56 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i56 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<7>, 7>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> u32 {
  // CHECK: %[[reg_a:.*]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %{{.*}}, i1 true)
  // CHECK: %[[reg_b:.*]] = trunc nuw nsw i64 %[[reg_a]] to i32
  // CHECK: ret i32 %[[reg_b]]
  unsafe { exint::backend::ctlz_nonzero::<uint<8>, 8>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 72) i32 @uint_9
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i72, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i72 0, 73) i72 @llvm.ctlz.i72(i72 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i72 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<9>, 9>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 80) i32 @uint_10
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i80, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i80 0, 81) i80 @llvm.ctlz.i80(i80 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i80 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<10>, 10>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 88) i32 @uint_11
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i88, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i88 0, 89) i88 @llvm.ctlz.i88(i88 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i88 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<11>, 11>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 96) i32 @uint_12
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i96, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i96 0, 97) i96 @llvm.ctlz.i96(i96 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i96 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<12>, 12>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 104) i32 @uint_13
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i104, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i104 0, 105) i104 @llvm.ctlz.i104(i104 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i104 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<13>, 13>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 112) i32 @uint_14
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i112, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i112 0, 113) i112 @llvm.ctlz.i112(i112 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i112 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<14>, 14>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 120) i32 @uint_15
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i120, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i120 0, 121) i120 @llvm.ctlz.i120(i120 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i120 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<15>, 15>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 128) i32 @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>) -> u32 {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = tail call range(i128 0, 129) i128 @llvm.ctlz.i128(i128 %[[reg_a]], i1 true)
  // CHECK: %[[reg_c:.*]] = trunc nuw nsw i128 %[[reg_b]] to i32
  // CHECK: ret i32 %[[reg_c]]
  unsafe { exint::backend::ctlz_nonzero::<uint<16>, 16>(a) }
}
