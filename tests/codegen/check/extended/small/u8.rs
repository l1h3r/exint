#![allow(non_camel_case_types)]

const N: usize = 1;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @band
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @bor
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @bxor
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @bnot
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i8 %[[A]], -1
  // CHECK: ret i8 %[[B]]
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i8 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @swap1
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i8 @llvm.bitreverse.i8(i8 %[[A]])
  // CHECK: ret i8 %[[B]]
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define i8 @swap8
// CHECK-SAME: (i8 returned %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: ret i8 %[[A]]
  exint::backend::swap8::<_, N>(a)
}

// CHECK-LABEL: define noundef i8 @rotl
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call noundef i8 @llvm.fshl.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  exint::backend::rotl::<_, N>(a, b)
}

// CHECK-LABEL: define noundef i8 @rotr
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = tail call noundef i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[A]], i8 %[[C]])
  // CHECK: ret i8 %[[D]]
  exint::backend::rotr::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @ctpop
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctpop.i8(i8 %[[A]])
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @ctlz
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 9) i32 @cttz
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @ctlz_nonzero
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.ctlz.i8(i8 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 8) i32 @cttz_nonzero
// CHECK-SAME: (i8 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i8 0, 9) i8 @llvm.cttz.i8(i8 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = zext nneg i8 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_uadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i8 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i8 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i16 256, i16 0
  // CHECK: %[[F:.+]] = zext i8 %[[C]] to i16
  // CHECK: %[[G:.+]] = or disjoint i16 %[[E]], %[[F]]
  // CHECK: ret i16 %[[G]]
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_usub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = sub i8 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i8 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i16 256, i16 0
  // CHECK: %[[F:.+]] = zext i8 %[[C]] to i16
  // CHECK: %[[G:.+]] = or disjoint i16 %[[E]], %[[F]]
  // CHECK: ret i16 %[[G]]
  exint::backend::overflowing_usub::<_, N>(a, b)
}

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_umul
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i8, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i8, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i16 256, i16 0
  // CHECK: %[[G:.+]] = zext i8 %[[D]] to i16
  // CHECK: %[[H:.+]] = or disjoint i16 %[[F]], %[[G]]
  // CHECK: ret i16 %[[H]]
  exint::backend::overflowing_umul::<_, N>(a, b)
}

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_sadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.sadd.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i8, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i8, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i16 256, i16 0
  // CHECK: %[[G:.+]] = zext i8 %[[D]] to i16
  // CHECK: %[[H:.+]] = or disjoint i16 %[[F]], %[[G]]
  // CHECK: ret i16 %[[H]]
  exint::backend::overflowing_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_ssub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.ssub.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i8, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i8, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i16 256, i16 0
  // CHECK: %[[G:.+]] = zext i8 %[[D]] to i16
  // CHECK: %[[H:.+]] = or disjoint i16 %[[F]], %[[G]]
  // CHECK: ret i16 %[[H]]
  exint::backend::overflowing_ssub::<_, N>(a, b)
}

// CHECK-LABEL: define range(i16 0, 512) i16 @overflowing_smul
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.smul.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i8, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i8, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i16 256, i16 0
  // CHECK: %[[G:.+]] = zext i8 %[[D]] to i16
  // CHECK: %[[H:.+]] = or disjoint i16 %[[F]], %[[G]]
  // CHECK: ret i16 %[[H]]
  exint::backend::overflowing_smul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @saturating_uadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.uadd.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @saturating_usub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.usub.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::saturating_usub::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @saturating_sadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.sadd.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::saturating_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @saturating_ssub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i8 @llvm.ssub.sat.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::saturating_ssub::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @unchecked_uadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_usub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_umul
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_udiv
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_udiv_exact
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i8 0, -1) i8 @unchecked_urem
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_sadd
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_ssub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_smul
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_sdiv
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_sdiv_exact
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i8 -127, -128) i8 @unchecked_srem
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::unchecked_srem::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_shl
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = shl i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { exint::backend::unchecked_shl::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_lshr
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = lshr i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { exint::backend::unchecked_lshr::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_ashr
// CHECK-SAME: (i8 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = trunc nuw i32 %[[B]] to i8
  // CHECK: %[[D:.+]] = ashr i8 %[[A]], %[[C]]
  // CHECK: ret i8 %[[D]]
  unsafe { exint::backend::unchecked_ashr::<_, N>(a, b) }
}

// CHECK-LABEL: define i8 @unchecked_fshl
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i8
  // CHECK: %[[E:.+]] = tail call i8 @llvm.fshl.i8(i8 %[[A]], i8 %[[B]], i8 %[[D]])
  // CHECK: ret i8 %[[E]]
  unsafe { exint::backend::unchecked_fshl::<_, N>(a, b, c) }
}

// CHECK-LABEL: define i8 @unchecked_fshr
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = trunc i32 %[[C]] to i8
  // CHECK: %[[E:.+]] = tail call i8 @llvm.fshr.i8(i8 %[[A]], i8 %[[B]], i8 %[[D]])
  // CHECK: ret i8 %[[E]]
  unsafe { exint::backend::unchecked_fshr::<_, N>(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @wrapping_add
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @wrapping_sub
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i8 %[[A]], %[[B]]
  // CHECK: ret i8 %[[C]]
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define i8 @wrapping_mul
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i8 @disjoint_bor
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i8 %[[B]], %[[A]]
  // CHECK: ret i8 %[[C]]
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
