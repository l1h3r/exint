#![allow(non_camel_case_types)]
#![feature(core_intrinsics)]
#![feature(funnel_shifts)]

type int = i8;
type uint = u8;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @band
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  a & b
}

// CHECK-LABEL: define noundef i8 @bor
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  a | b
}

// CHECK-LABEL: define noundef i8 @bxor
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  a ^ b
}

// CHECK-LABEL: define noundef i8 @bnot
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i8 %[[A]], -1
  // CHECK: ret i8 %[[B]]
  !a
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i8 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @swap1
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i8 @llvm.bitreverse.i8(i8 %[[A]])
  // CHECK: ret i8 %[[B]]
  ::core::intrinsics::bitreverse(a)
}

// CHECK-LABEL: define noundef i8 @swap8
// CHECK-SAME: (i8 noundef returned %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: ret i8 %[[A]]
  ::core::intrinsics::bswap(a)
}

// CHECK-LABEL: define noundef i8 @rotl
// CHECK-SAME: (i8 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call i8 @llvm.fshl.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  ::core::intrinsics::rotate_left(a, b)
}

// CHECK-LABEL: define noundef i8 @rotr
// CHECK-SAME: (i8 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  ::core::intrinsics::rotate_right(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @ctpop
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctpop.i8(i8 %[[A]])
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctpop(a)
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @ctlz
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctlz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @cttz
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::cttz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @ctlz_nonzero
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::ctlz_nonzero(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @cttz_nonzero
// CHECK-SAME: (i8 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::cttz_nonzero(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define { i8, i1 } @overflowing_uadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i8 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i8 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = insertvalue { i8, i1 } poison, i8 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i8, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i8, i1 } %[[F]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define { i8, i1 } @overflowing_usub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = sub i8 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i8 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = insertvalue { i8, i1 } poison, i8 %[[C]], 0
  // CHECK: %[[F:.+]] = insertvalue { i8, i1 } %[[E]], i1 %[[D]], 1
  // CHECK: ret { i8, i1 } %[[F]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i8, i1 } @overflowing_umul
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret { i8, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i8, i1 } @overflowing_sadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.sadd.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret { i8, i1 } %[[C]]
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i8, i1 } @overflowing_ssub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.ssub.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret { i8, i1 } %[[C]]
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define noundef { i8, i1 } @overflowing_smul
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.smul.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret { i8, i1 } %[[C]]
  ::core::intrinsics::mul_with_overflow(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @saturating_uadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.uadd.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i8 @saturating_usub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.usub.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// CHECK-LABEL: define noundef i8 @saturating_sadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.sadd.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i8 @saturating_ssub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ssub.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @unchecked_uadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_usub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_umul
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_udiv
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_udiv_exact
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i8 0, -1) i8 @unchecked_urem
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_sadd
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_ssub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_smul
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_sdiv
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_sdiv_exact
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i8 -127, -128) i8 @unchecked_srem
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_shl
// CHECK-SAME: (i8 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = shl i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shl(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_lshr
// CHECK-SAME: (i8 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = lshr i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_ashr
// CHECK-SAME: (i8 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = ashr i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i8 @unchecked_fshl
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i8
  // CHECK: %[[E:.+]] = tail call i8 @llvm.fshl.i8(i8 %[[A]], i8 %[[B]], i8 %[[D]])
  // CHECK: ret i8 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shl(a, b, c) }
}

// CHECK-LABEL: define noundef i8 @unchecked_fshr
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i8
  // CHECK: %[[E:.+]] = tail call i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[B]], i8 %[[D]])
  // CHECK: ret i8 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shr(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @wrapping_add
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::wrapping_add(a, b)
}

// CHECK-LABEL: define noundef i8 @wrapping_sub
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::wrapping_sub(a, b)
}

// CHECK-LABEL: define noundef i8 @wrapping_mul
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  ::core::intrinsics::wrapping_mul(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i8 @disjoint_bor
// CHECK-SAME: (i8 noundef %[[A:.+]], i8 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { ::core::intrinsics::disjoint_bitor(a, b) }
}
