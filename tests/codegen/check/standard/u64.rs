#![allow(non_camel_case_types)]
#![feature(core_intrinsics)]
#![feature(funnel_shifts)]

type int = i64;
type uint = u64;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @band
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  a & b
}

// CHECK-LABEL: define noundef i64 @bor
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  a | b
}

// CHECK-LABEL: define noundef i64 @bxor
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  a ^ b
}

// CHECK-LABEL: define noundef i64 @bnot
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i64 %[[A]], -1
  // CHECK: ret i64 %[[B]]
  !a
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i64 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @swap1
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i64 @llvm.bitreverse.i64(i64 %[[A]])
  // CHECK: ret i64 %[[B]]
  ::core::intrinsics::bitreverse(a)
}

// CHECK-LABEL: define noundef i64 @swap8
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i64 @llvm.bswap.i64(i64 %[[A]])
  // CHECK: ret i64 %[[B]]
  ::core::intrinsics::bswap(a)
}

// CHECK-LABEL: define noundef i64 @rotl
// CHECK-SAME: (i64 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call noundef i64 @llvm.fshl.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  ::core::intrinsics::rotate_left(a, b)
}

// CHECK-LABEL: define noundef i64 @rotr
// CHECK-SAME: (i64 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call noundef i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  ::core::intrinsics::rotate_right(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @ctpop
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %[[A]])
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctpop(a)
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @ctlz
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctlz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @cttz
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::cttz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @ctlz_nonzero
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::ctlz_nonzero(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @cttz_nonzero
// CHECK-SAME: (i64 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::cttz_nonzero(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define { i64, i1 } @overflowing_uadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i64 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i64 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = insertvalue { i64, i1 } poison, i64 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i64, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i64, i1 } %[[F]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define { i64, i1 } @overflowing_usub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = sub i64 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i64 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = insertvalue { i64, i1 } poison, i64 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i64, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i64, i1 } %[[F]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i64, i1 } @overflowing_umul
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret { i64, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i64, i1 } @overflowing_sadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret { i64, i1 } %[[C]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i64, i1 } @overflowing_ssub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret { i64, i1 } %[[C]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i64, i1 } @overflowing_smul
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i64, i1 } @llvm.smul.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret { i64, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @saturating_uadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.uadd.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i64 @saturating_usub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.usub.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// CHECK-LABEL: define noundef i64 @saturating_sadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.sadd.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i64 @saturating_ssub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.ssub.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @unchecked_uadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_usub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_umul
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_udiv
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_udiv_exact
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i64 0, -1) i64 @unchecked_urem
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_sadd
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_ssub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_smul
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_sdiv
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_sdiv_exact
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i64 -9223372036854775807, -9223372036854775808) i64 @unchecked_srem
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_shl
// CHECK-SAME: (i64 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = shl i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shl(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_lshr
// CHECK-SAME: (i64 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = lshr i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_ashr
// CHECK-SAME: (i64 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = ashr i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i64 @unchecked_fshl
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i64
  // CHECK: %[[E:.+]] = tail call i64 @llvm.fshl.i64(i64 %[[A]], i64 %[[B]], i64 %[[D]])
  // CHECK: ret i64 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shl(a, b, c) }
}

// CHECK-LABEL: define noundef i64 @unchecked_fshr
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i64
  // CHECK: %[[E:.+]] = tail call i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[B]], i64 %[[D]])
  // CHECK: ret i64 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shr(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @wrapping_add
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::wrapping_add(a, b)
}

// CHECK-LABEL: define noundef i64 @wrapping_sub
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::wrapping_sub(a, b)
}

// CHECK-LABEL: define noundef i64 @wrapping_mul
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  ::core::intrinsics::wrapping_mul(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i64 @disjoint_bor
// CHECK-SAME: (i64 noundef %[[A:.+]], i64 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { ::core::intrinsics::disjoint_bitor(a, b) }
}
