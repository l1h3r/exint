#![allow(non_camel_case_types)]

const N: usize = 4;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @band
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @bor
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @bxor
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @bnot
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i32 %[[A]], -1
  // CHECK: ret i32 %[[B]]
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i32 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @swap1
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i32 @llvm.bitreverse.i32(i32 %[[A]])
  // CHECK: ret i32 %[[B]]
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define i32 @swap8
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i32 @llvm.bswap.i32(i32 %[[A]])
  // CHECK: ret i32 %[[B]]
  exint::backend::swap8::<_, N>(a)
}

// CHECK-LABEL: define i32 @rotl
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.fshl.i32(i32 %[[A]], i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::rotl::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @rotr
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.fshr.i32(i32 %[[A]], i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::rotr::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @ctpop
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call noundef range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %[[A]])
  // CHECK: ret i32 %[[B]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @ctlz
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call noundef range(i32 0, 33) i32 @llvm.ctlz.i32(i32 %[[A]], i1 false)
  // CHECK: ret i32 %[[B]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 33) i32 @cttz
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call noundef range(i32 0, 33) i32 @llvm.cttz.i32(i32 %[[A]], i1 false)
  // CHECK: ret i32 %[[B]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 32) i32 @ctlz_nonzero
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call noundef range(i32 0, 32) i32 @llvm.ctlz.i32(i32 %[[A]], i1 true)
  // CHECK: ret i32 %[[B]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 32) i32 @cttz_nonzero
// CHECK-SAME: (i32 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call noundef range(i32 0, 32) i32 @llvm.cttz.i32(i32 %[[A]], i1 true)
  // CHECK: ret i32 %[[B]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_uadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = add i32 %[[B]], %[[A]]
  // CHECK: %[[D:.+]] = icmp ult i32 %[[C]], %[[A]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i40 4294967296, i40 0
  // CHECK: %[[F:.+]] = zext i32 %[[C]] to i40
  // CHECK: %[[G:.+]] = or disjoint i40 %[[E]], %[[F]]
  // CHECK: ret i40 %[[G]]
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_usub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = sub i32 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i32 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i40 4294967296, i40 0
  // CHECK: %[[F:.+]] = zext i32 %[[C]] to i40
  // CHECK: %[[G:.+]] = or disjoint i40 %[[E]], %[[F]]
  // CHECK: ret i40 %[[G]]
  exint::backend::overflowing_usub::<_, N>(a, b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_umul
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[C:.+]] = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i32, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i32, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i40 4294967296, i40 0
  // CHECK: %[[G:.+]] = zext i32 %[[D]] to i40
  // CHECK: %[[H:.+]] = or disjoint i40 %[[F]], %[[G]]
  // CHECK: ret i40 %[[H]]
  exint::backend::overflowing_umul::<_, N>(a, b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_sadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i32, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i32, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i40 4294967296, i40 0
  // CHECK: %[[G:.+]] = zext i32 %[[D]] to i40
  // CHECK: %[[H:.+]] = or disjoint i40 %[[F]], %[[G]]
  // CHECK: ret i40 %[[H]]
  exint::backend::overflowing_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_ssub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i32, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i32, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i40 4294967296, i40 0
  // CHECK: %[[G:.+]] = zext i32 %[[D]] to i40
  // CHECK: %[[H:.+]] = or disjoint i40 %[[F]], %[[G]]
  // CHECK: ret i40 %[[H]]
  exint::backend::overflowing_ssub::<_, N>(a, b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @overflowing_smul
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[C:.+]] = tail call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i32, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i32, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i40 4294967296, i40 0
  // CHECK: %[[G:.+]] = zext i32 %[[D]] to i40
  // CHECK: %[[H:.+]] = or disjoint i40 %[[F]], %[[G]]
  // CHECK: ret i40 %[[H]]
  exint::backend::overflowing_smul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @saturating_uadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.uadd.sat.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @saturating_usub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.usub.sat.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::saturating_usub::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @saturating_sadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.sadd.sat.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::saturating_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @saturating_ssub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i32 @llvm.ssub.sat.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: ret i32 %[[C]]
  exint::backend::saturating_ssub::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @unchecked_uadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_usub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_umul
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_udiv
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_udiv_exact
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i32 0, -1) i32 @unchecked_urem
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_sadd
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_ssub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_smul
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_sdiv
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_sdiv_exact
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i32 -2147483647, -2147483648) i32 @unchecked_srem
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_srem::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_shl
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = shl i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_shl::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_lshr
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = lshr i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_lshr::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_ashr
// CHECK-SAME: (i32 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = ashr i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::unchecked_ashr::<_, N>(a, b) }
}

// CHECK-LABEL: define i32 @unchecked_fshl
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = tail call i32 @llvm.fshl.i32(i32 %[[A]], i32 %[[B]], i32 %[[C]])
  // CHECK: ret i32 %[[D]]
  unsafe { exint::backend::unchecked_fshl::<_, N>(a, b, c) }
}

// CHECK-LABEL: define i32 @unchecked_fshr
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = tail call i32 @llvm.fshr.i32(i32 %[[A]], i32 %[[B]], i32 %[[C]])
  // CHECK: ret i32 %[[D]]
  unsafe { exint::backend::unchecked_fshr::<_, N>(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @wrapping_add
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @wrapping_sub
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i32 %[[A]], %[[B]]
  // CHECK: ret i32 %[[C]]
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define i32 @wrapping_mul
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i32 @disjoint_bor
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i32 %[[B]], %[[A]]
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
