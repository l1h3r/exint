#![allow(non_camel_case_types)]

const N: usize = 7;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i56 @band
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @bor
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @bxor
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @bnot
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i56 %[[A]], -1
  // CHECK: ret i56 %[[B]]
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i56 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i56 @swap1
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i56 @llvm.bitreverse.i56(i56 %[[A]])
  // CHECK: ret i56 %[[B]]
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define i56 @swap8
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = zext i56 %[[A]] to i64
  // CHECK: %[[C:.+]] = tail call i64 @llvm.bswap.i64(i64 %[[B]])
  // CHECK: %[[D:.+]] = lshr exact i64 %[[C]], 8
  // CHECK: %[[E:.+]] = trunc nuw i64 %[[D]] to i56
  // CHECK: ret i56 %[[E]]
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

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @ctpop
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.ctpop.i56(i56 %[[A]])
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @ctlz
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.ctlz.i56(i56 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 57) i32 @cttz
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.cttz.i56(i56 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 56) i32 @ctlz_nonzero
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.ctlz.i56(i56 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 56) i32 @cttz_nonzero
// CHECK-SAME: (i56 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i56 0, 57) i56 @llvm.cttz.i56(i56 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i56 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define range(i64 0, 144115188075855872) i64 @overflowing_uadd
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i56 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i56 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i64 72057594037927936, i64 0
  // CHECK: %[[F:.+]] = zext i56 %[[C]] to i64
  // CHECK: %[[G:.+]] = or disjoint i64 %[[E]], %[[F]]
  // CHECK: ret i64 %[[G]]
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i64 0, 144115188075855872) i64 @overflowing_usub
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = icmp ult i56 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = select i1 %[[C]], i64 72057594037927936, i64 0
  // CHECK: %[[E:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK: %[[F:.+]] = zext i56 %[[E]] to i64
  // CHECK: %[[G:.+]] = or disjoint i64 %[[D]], %[[F]]
  // CHECK: ret i64 %[[G]]
  exint::backend::overflowing_usub::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
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

// CHECK-LABEL: define i56 @saturating_uadd
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i56 @llvm.uadd.sat.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i56 %[[C]]
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @saturating_usub
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i56 @llvm.usub.sat.i56(i56 %[[A]], i56 %[[B]])
  // CHECK: ret i56 %[[C]]
  exint::backend::saturating_usub::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  exint::backend::saturating_sadd::<_, N>(a, b)
}

// TODO
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  exint::backend::saturating_ssub::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i56 @unchecked_uadd
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_usub
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_umul
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_udiv
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_udiv_exact
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i56 0, -1) i56 @unchecked_urem
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_sadd
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_ssub
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_smul
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_sdiv
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i56 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i56 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i56
  // CHECK: ret i56 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i56 @unchecked_sdiv_exact
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i56 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i56 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv exact i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i56
  // CHECK: ret i56 %[[F]]
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i56 -36028797018963967, -36028797018963968) i56 @unchecked_srem
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i56 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i56 %[[B]] to i64
  // CHECK: %[[E:.+]] = srem i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc nsw i64 %[[E]] to i56
  // CHECK: ret i56 %[[F]]
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

// CHECK-LABEL: define i56 @wrapping_add
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @wrapping_sub
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK: ret i56 %[[C]]
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define i56 @wrapping_mul
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i56 @disjoint_bor
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i56 %[[B]], %[[A]]
  // CHECK: ret i56 %[[C]]
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
