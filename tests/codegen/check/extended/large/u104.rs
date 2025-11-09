#![allow(non_camel_case_types)]

const N: usize = 13;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @band
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = and i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define void @bor
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = or i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define void @bxor
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = xor i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define void @bnot
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = xor i104 %[[C]], -1
  // CHECK: store i104 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = icmp eq i104 %[[C]], %[[D]]
  // CHECK: ret i1 %[[E]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i104(i104 %[[C]], i104 %[[D]])
  // CHECK: ret i8 %[[E]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[E:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i104(i104 %[[C]], i104 %[[D]])
  // CHECK: ret i8 %[[E]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @swap1
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = tail call i104 @llvm.bitreverse.i104(i104 %[[C]])
  // CHECK: store i104 %[[D]], ptr %[[B]], align 1
  // CHECK: ret void
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define void @swap8
// CHECK-SAME: (ptr {{.*}} %[[B:.+]], ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[C:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[D:.+]] = zext i104 %[[C]] to i128
  // CHECK: %[[E:.+]] = tail call i128 @llvm.bswap.i128(i128 %[[D]])
  // CHECK: %[[F:.+]] = lshr exact i128 %[[E]], 24
  // CHECK: %[[G:.+]] = trunc nuw i128 %[[F]] to i104
  // CHECK: store i104 %[[G]], ptr %[[B]], align 1
  // CHECK: ret void
  exint::backend::swap8::<_, N>(a)
}

// TODO
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  exint::backend::rotl::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  exint::backend::rotr::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @ctpop
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.ctpop.i104(i104 %[[B]])
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @ctlz
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.ctlz.i104(i104 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 105) i32 @cttz
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.cttz.i104(i104 %[[B]], i1 false)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 104) i32 @ctlz_nonzero
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.ctlz.i104(i104 %[[B]], i1 true)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 104) i32 @cttz_nonzero
// CHECK-SAME: (ptr {{.*}} %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[C:.+]] = tail call range(i104 0, 105) i104 @llvm.cttz.i104(i104 %[[B]], i1 true)
  // CHECK: %[[D:.+]] = trunc nuw nsw i104 %[[C]] to i32
  // CHECK: ret i32 %[[D]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @overflowing_uadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = add i104 %[[E]], %[[D]]
  // CHECK: %[[G:.+]] = icmp ult i104 %[[F]], %[[D]]
  // CHECK: %[[H:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[I:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 13
  // CHECK: store i8 %[[H]], ptr %[[I]], align 1
  // CHECK: ret void
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_usub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = icmp ult i104 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i104 %[[G]], ptr %[[C]], align 1
  // CHECK: %[[I:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 13
  // CHECK: store i8 %[[H]], ptr %[[I]], align 1
  // CHECK: ret void
  exint::backend::overflowing_usub::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_umul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = freeze i104 %[[D]]
  // CHECK: %[[G:.+]] = tail call { i104, i1 } @llvm.umul.with.overflow.i104(i104 %[[E]], i104 %[[F]])
  // CHECK: %[[H:.+]] = extractvalue { i104, i1 } %[[G]], 1
  // CHECK: %[[I:.+]] = zext i1 %[[H]] to i8
  // CHECK: %[[J:.+]] = extractvalue { i104, i1 } %[[G]], 0
  // CHECK: store i104 %[[J]], ptr %[[C]], align 1
  // CHECK: %[[K:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 13
  // CHECK: store i8 %[[I]], ptr %[[K]], align 1
  // CHECK: ret void
  exint::backend::overflowing_umul::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  exint::backend::overflowing_sadd::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  exint::backend::overflowing_ssub::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  exint::backend::overflowing_smul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @saturating_uadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i104 @llvm.uadd.sat.i104(i104 %[[D]], i104 %[[E]])
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define void @saturating_usub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i104 @llvm.usub.sat.i104(i104 %[[D]], i104 %[[E]])
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::saturating_usub::<_, N>(a, b)
}

// CHECK-LABEL: define void @saturating_sadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i104 @llvm.sadd.sat.i104(i104 %[[E]], i104 %[[D]])
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::saturating_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define void @saturating_ssub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call i104 @llvm.ssub.sat.i104(i104 %[[D]], i104 %[[E]])
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::saturating_ssub::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @unchecked_uadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = add i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_usub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_umul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_udiv
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = udiv i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_udiv_exact
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = udiv i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_urem
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = urem i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_sadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = add i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_ssub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_smul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_sdiv
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i104 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i104 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i104
  // CHECK: store i104 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_sdiv_exact
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i104 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i104 %[[E]] to i128
  // CHECK: %[[H:.+]] = sdiv exact i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc i128 %[[H]] to i104
  // CHECK: store i104 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define void @unchecked_srem
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sext i104 %[[D]] to i128
  // CHECK: %[[G:.+]] = sext i104 %[[E]] to i128
  // CHECK: %[[H:.+]] = srem i128 %[[F]], %[[G]]
  // CHECK: %[[I:.+]] = trunc nsw i128 %[[H]] to i104
  // CHECK: store i104 %[[I]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::unchecked_srem::<_, N>(a, b) }
}

// TODO
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  unsafe { exint::backend::unchecked_shl::<_, N>(a, b) }
}

// TODO
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  unsafe { exint::backend::unchecked_lshr::<_, N>(a, b) }
}

// TODO
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  unsafe { exint::backend::unchecked_ashr::<_, N>(a, b) }
}

// TODO
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  unsafe { exint::backend::unchecked_fshl::<_, N>(a, b, c) }
}

// TODO
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  unsafe { exint::backend::unchecked_fshr::<_, N>(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @wrapping_add
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = add i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define void @wrapping_sub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define void @wrapping_mul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = mul i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @disjoint_bor
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = or disjoint i104 %[[E]], %[[D]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: ret void
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
