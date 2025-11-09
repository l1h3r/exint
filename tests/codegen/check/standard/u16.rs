#![allow(non_camel_case_types)]
#![feature(core_intrinsics)]
#![feature(funnel_shifts)]

type int = i16;
type uint = u16;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @band
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  a & b
}

// CHECK-LABEL: define noundef i16 @bor
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  a | b
}

// CHECK-LABEL: define noundef i16 @bxor
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  a ^ b
}

// CHECK-LABEL: define noundef i16 @bnot
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i16 %[[A]], -1
  // CHECK: ret i16 %[[B]]
  !a
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i16 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @swap1
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i16 @llvm.bitreverse.i16(i16 %[[A]])
  // CHECK: ret i16 %[[B]]
  ::core::intrinsics::bitreverse(a)
}

// CHECK-LABEL: define noundef i16 @swap8
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i16 @llvm.bswap.i16(i16 %[[A]])
  // CHECK: ret i16 %[[B]]
  ::core::intrinsics::bswap(a)
}

// CHECK-LABEL: define noundef i16 @rotl
// CHECK-SAME: (i16 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = tail call i16 @llvm.fshl.i16(i16 %[[A]], i16 %[[A]], i16 %[[C]])
  // CHECK: ret i16 %[[D]]
  ::core::intrinsics::rotate_left(a, b)
}

// CHECK-LABEL: define noundef i16 @rotr
// CHECK-SAME: (i16 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = tail call i16 @llvm.fshr.i16(i16 %[[A]], i16 %[[A]], i16 %[[C]])
  // CHECK: ret i16 %[[D]]
  ::core::intrinsics::rotate_right(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @ctpop
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.ctpop.i16(i16 %[[A]])
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctpop(a)
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @ctlz
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.ctlz.i16(i16 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctlz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 17) i32 @cttz
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::cttz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 16) i32 @ctlz_nonzero
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.ctlz.i16(i16 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::ctlz_nonzero(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 16) i32 @cttz_nonzero
// CHECK-SAME: (i16 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i16 0, 17) i16 @llvm.cttz.i16(i16 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i16 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::cttz_nonzero(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define { i16, i1 } @overflowing_uadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i16 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i16 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = insertvalue { i16, i1 } poison, i16 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i16, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i16, i1 } %[[F]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define { i16, i1 } @overflowing_usub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = sub i16 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i16 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = insertvalue { i16, i1 } poison, i16 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i16, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i16, i1 } %[[F]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i16, i1 } @overflowing_umul
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = tail call { i16, i1 } @llvm.umul.with.overflow.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret { i16, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i16, i1 } @overflowing_sadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i16, i1 } @llvm.sadd.with.overflow.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret { i16, i1 } %[[C]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i16, i1 } @overflowing_ssub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i16, i1 } @llvm.ssub.with.overflow.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret { i16, i1 } %[[C]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i16, i1 } @overflowing_smul
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i16, i1 } @llvm.smul.with.overflow.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret { i16, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @saturating_uadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i16 @llvm.uadd.sat.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i16 @saturating_usub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i16 @llvm.usub.sat.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// CHECK-LABEL: define noundef i16 @saturating_sadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i16 @llvm.sadd.sat.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i16 @saturating_ssub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i16 @llvm.ssub.sat.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @unchecked_uadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_usub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_umul
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_udiv
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_udiv_exact
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i16 0, -1) i16 @unchecked_urem
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_sadd
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_ssub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_smul
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_sdiv
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_sdiv_exact
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i16 -32767, -32768) i16 @unchecked_srem
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_shl
// CHECK-SAME: (i16 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = shl i16 %[[A]], %[[C]]
  // CHECK: ret i16 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shl(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_lshr
// CHECK-SAME: (i16 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = lshr i16 %[[A]], %[[C]]
  // CHECK: ret i16 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_ashr
// CHECK-SAME: (i16 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i16
  // CHECK: %[[D:.+]] = ashr i16 %[[A]], %[[C]]
  // CHECK: ret i16 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i16 @unchecked_fshl
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i16
  // CHECK: %[[E:.+]] = tail call i16 @llvm.fshl.i16(i16 %[[A]], i16 %[[B]], i16 %[[D]])
  // CHECK: ret i16 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shl(a, b, c) }
}

// CHECK-LABEL: define noundef i16 @unchecked_fshr
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i16
  // CHECK: %[[E:.+]] = tail call i16 @llvm.fshr.i16(i16 %[[A]], i16 %[[B]], i16 %[[D]])
  // CHECK: ret i16 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shr(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @wrapping_add
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::wrapping_add(a, b)
}

// CHECK-LABEL: define noundef i16 @wrapping_sub
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i16 %[[A]], %[[B]]
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::wrapping_sub(a, b)
}

// CHECK-LABEL: define noundef i16 @wrapping_mul
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  ::core::intrinsics::wrapping_mul(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i16 @disjoint_bor
// CHECK-SAME: (i16 noundef %[[A:.+]], i16 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i16 %[[B]], %[[A]]
  // CHECK: ret i16 %[[C]]
  unsafe { ::core::intrinsics::disjoint_bitor(a, b) }
}
