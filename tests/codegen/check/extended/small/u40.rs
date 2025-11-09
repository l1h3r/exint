#![allow(non_camel_case_types)]

const N: usize = 5;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i40 @band
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @bor
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @bxor
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @bnot
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i40 %[[A]], -1
  // CHECK: ret i40 %[[B]]
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i40 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i40 @swap1
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i40 @llvm.bitreverse.i40(i40 %[[A]])
  // CHECK: ret i40 %[[B]]
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define i40 @swap8
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = zext i40 %[[A]] to i64
  // CHECK: %[[C:.+]] = tail call i64 @llvm.bswap.i64(i64 %[[B]])
  // CHECK: %[[D:.+]] = lshr exact i64 %[[C]], 24
  // CHECK: %[[E:.+]] = trunc nuw i64 %[[D]] to i40
  // CHECK: ret i40 %[[E]]
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

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @ctpop
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.ctpop.i40(i40 %[[A]])
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @ctlz
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.ctlz.i40(i40 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 41) i32 @cttz
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.cttz.i40(i40 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 40) i32 @ctlz_nonzero
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.ctlz.i40(i40 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 40) i32 @cttz_nonzero
// CHECK-SAME: (i40 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i40 0, 41) i40 @llvm.cttz.i40(i40 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i40 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define range(i48 0, 2199023255552) i48 @overflowing_uadd
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i40 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i40 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i48 1099511627776, i48 0
  // CHECK: %[[F:.+]] = zext i40 %[[C]] to i48
  // CHECK: %[[G:.+]] = or disjoint i48 %[[E]], %[[F]]
  // CHECK: ret i48 %[[G]]
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i48 0, 2199023255552) i48 @overflowing_usub
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = icmp ult i40 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = select i1 %[[C]], i48 1099511627776, i48 0
  // CHECK: %[[E:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK: %[[F:.+]] = zext i40 %[[E]] to i48
  // CHECK: %[[G:.+]] = or disjoint i48 %[[D]], %[[F]]
  // CHECK: ret i48 %[[G]]
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

// CHECK-LABEL: define i40 @saturating_uadd
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i40 @llvm.uadd.sat.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i40 %[[C]]
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @saturating_usub
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i40 @llvm.usub.sat.i40(i40 %[[A]], i40 %[[B]])
  // CHECK: ret i40 %[[C]]
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

// CHECK-LABEL: define i40 @unchecked_uadd
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_usub
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_umul
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_udiv
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_udiv_exact
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i40 0, -1) i40 @unchecked_urem
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_sadd
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_ssub
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_smul
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_sdiv
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i40 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i40 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i40
  // CHECK: ret i40 %[[F]]
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i40 @unchecked_sdiv_exact
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i40 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i40 %[[B]] to i64
  // CHECK: %[[E:.+]] = sdiv exact i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc i64 %[[E]] to i40
  // CHECK: ret i40 %[[F]]
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i40 -549755813887, -549755813888) i40 @unchecked_srem
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sext i40 %[[A]] to i64
  // CHECK: %[[D:.+]] = sext i40 %[[B]] to i64
  // CHECK: %[[E:.+]] = srem i64 %[[C]], %[[D]]
  // CHECK: %[[F:.+]] = trunc nsw i64 %[[E]] to i40
  // CHECK: ret i40 %[[F]]
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

// CHECK-LABEL: define i40 @wrapping_add
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @wrapping_sub
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK: ret i40 %[[C]]
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define i40 @wrapping_mul
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i40 @disjoint_bor
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i40 %[[B]], %[[A]]
  // CHECK: ret i40 %[[C]]
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
