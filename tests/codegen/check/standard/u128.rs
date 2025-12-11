#![allow(non_camel_case_types)]
#![feature(core_intrinsics)]
#![feature(funnel_shifts)]

type int = i128;
type uint = u128;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @band
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  a & b
}

// CHECK-LABEL: define noundef i128 @bor
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  a | b
}

// CHECK-LABEL: define noundef i128 @bxor
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  a ^ b
}

// CHECK-LABEL: define noundef i128 @bnot
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i128 %[[A]], -1
  // CHECK: ret i128 %[[B]]
  !a
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i128 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  a == b
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ucmp.i8.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.scmp.i8.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i8 %[[C]]
  a.cmp(&b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @swap1
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i128 @llvm.bitreverse.i128(i128 %[[A]])
  // CHECK: ret i128 %[[B]]
  ::core::intrinsics::bitreverse(a)
}

// CHECK-LABEL: define noundef i128 @swap8
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i128 @llvm.bswap.i128(i128 %[[A]])
  // CHECK: ret i128 %[[B]]
  ::core::intrinsics::bswap(a)
}

// CHECK-LABEL: define noundef i128 @rotl
// CHECK-SAME: (i128 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i128
  // CHECK: %[[D:.+]] = tail call noundef i128 @llvm.fshl.i128(i128 %[[A]], i128 %[[A]], i128 %[[C]])
  // CHECK: ret i128 %[[D]]
  ::core::intrinsics::rotate_left(a, b)
}

// CHECK-LABEL: define noundef i128 @rotr
// CHECK-SAME: (i128 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i128
  // CHECK: %[[D:.+]] = tail call noundef i128 @llvm.fshr.i128(i128 %[[A]], i128 %[[A]], i128 %[[C]])
  // CHECK: ret i128 %[[D]]
  ::core::intrinsics::rotate_right(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @ctpop
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i128 0, 129) i128 @llvm.ctpop.i128(i128 %[[A]])
  // CHECK: %[[C:.+]] = trunc nuw nsw i128 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctpop(a)
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @ctlz
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i128 0, 129) i128 @llvm.ctlz.i128(i128 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i128 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::ctlz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 129) i32 @cttz
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i128 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  ::core::intrinsics::cttz(a)
}

// CHECK-LABEL: define noundef range(i32 0, 128) i32 @ctlz_nonzero
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i128 0, 129) i128 @llvm.ctlz.i128(i128 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i128 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::ctlz_nonzero(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 128) i32 @cttz_nonzero
// CHECK-SAME: (i128 noundef %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i128 0, 129) i128 @llvm.cttz.i128(i128 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i128 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { ::core::intrinsics::cttz_nonzero(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @overflowing_uadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = add i128 %[[B]], %[[A]]
  // CHECK: %[[E:.+]] = icmp ult i128 %[[D]], %[[A]]
  // CHECK: store i128 %[[D]], ptr %[[C]], align 16
  // CHECK: %[[F:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[G:.+]] = zext i1 %[[E]] to i8
  // CHECK: store i8 %[[G]], ptr %[[F]], align 16
  // CHECK: ret void
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define void @overflowing_usub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = sub i128 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = icmp ult i128 %[[A]], %[[B]]
  // CHECK: store i128 %[[D]], ptr %[[C]], align 16
  // CHECK: %[[F:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[G:.+]] = zext i1 %[[E]] to i8
  // CHECK: store i8 %[[G]], ptr %[[F]], align 16
  // CHECK: ret void
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define void @overflowing_umul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = tail call { i128, i1 } @llvm.umul.with.overflow.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i128, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i128, i1 } %[[D]], 1
  // CHECK: store i128 %[[E]], ptr %[[C]], align 16
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 16
  // CHECK: ret void
  ::core::intrinsics::mul_with_overflow(a, b)
}

// CHECK-LABEL: define void @overflowing_sadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i128, i1 } @llvm.sadd.with.overflow.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i128, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i128, i1 } %[[D]], 1
  // CHECK: store i128 %[[E]], ptr %[[C]], align 16
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 16
  // CHECK: ret void
  ::core::intrinsics::add_with_overflow(a, b)
}

// CHECK-LABEL: define void @overflowing_ssub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i128, i1 } @llvm.ssub.with.overflow.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i128, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i128, i1 } %[[D]], 1
  // CHECK: store i128 %[[E]], ptr %[[C]], align 16
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 16
  // CHECK: ret void
  ::core::intrinsics::sub_with_overflow(a, b)
}

// CHECK-LABEL: define void @overflowing_smul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i128, i1 } @llvm.smul.with.overflow.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i128, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i128, i1 } %[[D]], 1
  // CHECK: store i128 %[[E]], ptr %[[C]], align 16
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 16
  // CHECK: ret void
  ::core::intrinsics::mul_with_overflow(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @saturating_uadd
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i128 @llvm.uadd.sat.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i128 @saturating_usub
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i128 @llvm.usub.sat.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// CHECK-LABEL: define noundef i128 @saturating_sadd
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i128 @llvm.sadd.sat.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::saturating_add(a, b)
}

// CHECK-LABEL: define noundef i128 @saturating_ssub
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i128 @llvm.ssub.sat.i128(i128 %[[A]], i128 %[[B]])
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::saturating_sub(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @unchecked_uadd
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_usub
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_umul
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_udiv
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_udiv_exact
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i128 0, -1) i128 @unchecked_urem
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_sadd
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_add(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_ssub
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_sub(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_smul
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_mul(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_sdiv
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_div(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_sdiv_exact
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::exact_div(a, b) }
}

// CHECK-LABEL: define noundef range(i128 -170141183460469231731687303715884105727, -170141183460469231731687303715884105728) i128 @unchecked_srem
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::unchecked_rem(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_shl
// CHECK-SAME: (i128 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i128
  // CHECK: %[[D:.+]] = shl i128 %[[A]], %[[C]]
  // CHECK: ret i128 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shl(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_lshr
// CHECK-SAME: (i128 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i128
  // CHECK: %[[D:.+]] = lshr i128 %[[A]], %[[C]]
  // CHECK: ret i128 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_ashr
// CHECK-SAME: (i128 noundef %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i128
  // CHECK: %[[D:.+]] = ashr i128 %[[A]], %[[C]]
  // CHECK: ret i128 %[[D]]
  unsafe { ::core::intrinsics::unchecked_shr(a, b) }
}

// CHECK-LABEL: define noundef i128 @unchecked_fshl
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i128
  // CHECK: %[[E:.+]] = tail call i128 @llvm.fshl.i128(i128 %[[A]], i128 %[[B]], i128 %[[D]])
  // CHECK: ret i128 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shl(a, b, c) }
}

// CHECK-LABEL: define noundef i128 @unchecked_fshr
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i128
  // CHECK: %[[E:.+]] = tail call i128 @llvm.fshr.i128(i128 %[[A]], i128 %[[B]], i128 %[[D]])
  // CHECK: ret i128 %[[E]]
  unsafe { ::core::intrinsics::unchecked_funnel_shr(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @wrapping_add
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::wrapping_add(a, b)
}

// CHECK-LABEL: define noundef i128 @wrapping_sub
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i128 %[[A]], %[[B]]
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::wrapping_sub(a, b)
}

// CHECK-LABEL: define noundef i128 @wrapping_mul
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  ::core::intrinsics::wrapping_mul(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef i128 @disjoint_bor
// CHECK-SAME: (i128 noundef %[[A:.+]], i128 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i128 %[[B]], %[[A]]
  // CHECK: ret i128 %[[C]]
  unsafe { ::core::intrinsics::disjoint_bitor(a, b) }
}
