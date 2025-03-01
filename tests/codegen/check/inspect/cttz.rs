use exint::int;
use exint::uint;

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @int_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_1(a: int<1>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @int_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_2(a: int<2>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 25) i32 @int_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_3(a: int<3>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i24 0, 25) i24 @llvm.cttz.i24(i24 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i24 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @int_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_4(a: int<4>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i32 0, 33) i32 @llvm.cttz.i32(i32 %[[A]], i1 false)
  // CHECK: ret i32 %[[B]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @int_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_5(a: int<5>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.cttz.i40(i40 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 49) i32 @int_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_6(a: int<6>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i48 0, 49) i48 @llvm.cttz.i48(i48 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i48 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @int_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_7(a: int<7>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.cttz.i56(i56 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @int_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_8(a: int<8>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 73) i32 @int_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_9(a: int<9>) -> u32 {
  // CHECK: %[[B:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i72 0, 73) i72 @llvm.cttz.i72(i72 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i72 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 81) i32 @int_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_10(a: int<10>) -> u32 {
  // CHECK: %[[B:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i80 0, 81) i80 @llvm.cttz.i80(i80 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i80 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 89) i32 @int_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_11(a: int<11>) -> u32 {
  // CHECK: %[[B:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i88 0, 89) i88 @llvm.cttz.i88(i88 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i88 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 97) i32 @int_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_12(a: int<12>) -> u32 {
  // CHECK: %[[B:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i96 0, 97) i96 @llvm.cttz.i96(i96 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i96 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @int_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_13(a: int<13>) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.cttz.i104(i104 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 113) i32 @int_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_14(a: int<14>) -> u32 {
  // CHECK: %[[B:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i112 0, 113) i112 @llvm.cttz.i112(i112 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i112 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 121) i32 @int_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_15(a: int<15>) -> u32 {
  // CHECK: %[[B:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i120 0, 121) i120 @llvm.cttz.i120(i120 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i120 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @int_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn int_16(a: int<16>) -> u32 {
  // CHECK: %[[B:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i128 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @uint_1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @uint_2
// CHECK-SAME: (i16 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 25) i32 @uint_3
// CHECK-SAME: (i24 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i24 0, 25) i24 @llvm.cttz.i24(i24 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i24 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @uint_4
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i32 0, 33) i32 @llvm.cttz.i32(i32 %[[A]], i1 false)
  // CHECK: ret i32 %[[B]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @uint_5
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.cttz.i40(i40 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 49) i32 @uint_6
// CHECK-SAME: (i48 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i48 0, 49) i48 @llvm.cttz.i48(i48 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i48 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @uint_7
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.cttz.i56(i56 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @uint_8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 73) i32 @uint_9
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>) -> u32 {
  // CHECK: %[[B:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i72 0, 73) i72 @llvm.cttz.i72(i72 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i72 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 81) i32 @uint_10
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>) -> u32 {
  // CHECK: %[[B:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i80 0, 81) i80 @llvm.cttz.i80(i80 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i80 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 89) i32 @uint_11
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>) -> u32 {
  // CHECK: %[[B:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i88 0, 89) i88 @llvm.cttz.i88(i88 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i88 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 97) i32 @uint_12
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>) -> u32 {
  // CHECK: %[[B:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i96 0, 97) i96 @llvm.cttz.i96(i96 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i96 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @uint_13
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.cttz.i104(i104 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 113) i32 @uint_14
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>) -> u32 {
  // CHECK: %[[B:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i112 0, 113) i112 @llvm.cttz.i112(i112 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i112 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 121) i32 @uint_15
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>) -> u32 {
  // CHECK: %[[B:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i120 0, 121) i120 @llvm.cttz.i120(i120 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i120 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @uint_16
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>) -> u32 {
  // CHECK: %[[B:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i128 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  a.trailing_zeros()
}
