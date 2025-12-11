#![allow(non_camel_case_types)]

const N: usize = 8;

type int = exint::uint<N>;
type uint = exint::uint<N>;

// -----------------------------------------------------------------------------
// Bitwise Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @band
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn band(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = and i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  exint::backend::band::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @bor
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  exint::backend::bor::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @bxor
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn bxor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = xor i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  exint::backend::bxor::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @bnot
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn bnot(a: uint) -> uint {
  // CHECK: %[[B:.+]] = xor i64 %[[A]], -1
  // CHECK: ret i64 %[[B]]
  exint::backend::bnot::<_, N>(a)
}

// -----------------------------------------------------------------------------
// Comparison Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef zeroext i1 @eq
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn eq(a: uint, b: uint) -> bool {
  // CHECK: %[[C:.+]] = icmp eq i64 %[[A]], %[[B]]
  // CHECK: ret i1 %[[C]]
  exint::backend::eq::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @ucmp
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn ucmp(a: uint, b: uint) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.ucmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::ucmp::<_, N>(a, b)
}

// CHECK-LABEL: define noundef range(i8 -1, 2) i8 @scmp
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn scmp(a: int, b: int) -> ::core::cmp::Ordering {
  // CHECK: %[[C:.+]] = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i8 %[[C]]
  exint::backend::scmp::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Conversion Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @swap1
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap1(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i64 @llvm.bitreverse.i64(i64 %[[A]])
  // CHECK: ret i64 %[[B]]
  exint::backend::swap1::<_, N>(a)
}

// CHECK-LABEL: define i64 @swap8
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn swap8(a: uint) -> uint {
  // CHECK: %[[B:.+]] = tail call i64 @llvm.bswap.i64(i64 %[[A]])
  // CHECK: ret i64 %[[B]]
  exint::backend::swap8::<_, N>(a)
}

// CHECK-LABEL: define noundef i64 @rotl
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call noundef i64 @llvm.fshl.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  exint::backend::rotl::<_, N>(a, b)
}

// CHECK-LABEL: define noundef i64 @rotr
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn rotr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = tail call noundef i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[A]], i64 %[[C]])
  // CHECK: ret i64 %[[D]]
  exint::backend::rotr::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Bit Inspection Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @ctpop
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctpop(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctpop.i64(i64 %[[A]])
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctpop::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @ctlz
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::ctlz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 65) i32 @cttz
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 false)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  exint::backend::cttz::<_, N>(a)
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @ctlz_nonzero
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn ctlz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::ctlz_nonzero::<_, N>(a) }
}

// CHECK-LABEL: define noundef range(i32 0, 64) i32 @cttz_nonzero
// CHECK-SAME: (i64 %[[A:.+]])
#[unsafe(no_mangle)]
pub fn cttz_nonzero(a: uint) -> u32 {
  // CHECK: %[[B:.+]] = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %[[A]], i1 true)
  // CHECK: %[[C:.+]] = trunc nuw nsw i64 %[[B]] to i32
  // CHECK: ret i32 %[[C]]
  unsafe { exint::backend::cttz_nonzero::<_, N>(a) }
}

// -----------------------------------------------------------------------------
// Overflowing Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define void @overflowing_uadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_uadd(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = add i64 %[[B]], %[[A]]
  // CHECK: %[[E:.+]] = icmp ult i64 %[[D]], %[[A]]
  // CHECK: store i64 %[[D]], ptr %[[C]], align 1
  // CHECK: %[[F:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[G:.+]] = zext i1 %[[E]] to i8
  // CHECK: store i8 %[[G]], ptr %[[F]], align 1
  // CHECK: ret void
  exint::backend::overflowing_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_usub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_usub(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = sub i64 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = icmp ult i64 %[[A]], %[[B]]
  // CHECK: store i64 %[[D]], ptr %[[C]], align 1
  // CHECK: %[[F:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[G:.+]] = zext i1 %[[E]] to i8
  // CHECK: store i8 %[[G]], ptr %[[F]], align 1
  // CHECK: ret void
  exint::backend::overflowing_usub::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_umul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_umul(a: uint, b: uint) -> (uint, bool) {
  // CHECK: %[[D:.+]] = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i64, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i64, i1 } %[[D]], 1
  // CHECK: store i64 %[[E]], ptr %[[C]], align 1
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 1
  // CHECK: ret void
  exint::backend::overflowing_umul::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_sadd
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_sadd(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i64, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i64, i1 } %[[D]], 1
  // CHECK: store i64 %[[E]], ptr %[[C]], align 1
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 1
  // CHECK: ret void
  exint::backend::overflowing_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_ssub
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_ssub(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i64, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i64, i1 } %[[D]], 1
  // CHECK: store i64 %[[E]], ptr %[[C]], align 1
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 1
  // CHECK: ret void
  exint::backend::overflowing_ssub::<_, N>(a, b)
}

// CHECK-LABEL: define void @overflowing_smul
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn overflowing_smul(a: int, b: int) -> (int, bool) {
  // CHECK: %[[D:.+]] = tail call { i64, i1 } @llvm.smul.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i64, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i64, i1 } %[[D]], 1
  // CHECK: store i64 %[[E]], ptr %[[C]], align 1
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 1
  // CHECK: ret void
  exint::backend::overflowing_smul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Saturating Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @saturating_uadd
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.uadd.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  exint::backend::saturating_uadd::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @saturating_usub
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.usub.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  exint::backend::saturating_usub::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @saturating_sadd
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.sadd.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  exint::backend::saturating_sadd::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @saturating_ssub
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn saturating_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = tail call i64 @llvm.ssub.sat.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: ret i64 %[[C]]
  exint::backend::saturating_ssub::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Unchecked Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @unchecked_uadd
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_uadd(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add nuw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_uadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_usub
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_usub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub nuw i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_usub::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_umul
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_umul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul nuw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_umul::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_udiv
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_udiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_udiv_exact
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_udiv_exact(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = udiv exact i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_udiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i64 0, -1) i64 @unchecked_urem
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_urem(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = urem i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_urem::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_sadd
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sadd(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = add nsw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_sadd::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_ssub
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ssub(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sub nsw i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_ssub::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_smul
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_smul(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = mul nsw i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_smul::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_sdiv
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_sdiv::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_sdiv_exact
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_sdiv_exact(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = sdiv exact i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_sdiv_exact::<_, N>(a, b) }
}

// CHECK-LABEL: define range(i64 -9223372036854775807, -9223372036854775808) i64 @unchecked_srem
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_srem(a: int, b: int) -> int {
  // CHECK: %[[C:.+]] = srem i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::unchecked_srem::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_shl
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_shl(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = shl i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { exint::backend::unchecked_shl::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_lshr
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_lshr(a: uint, b: u32) -> uint {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = lshr i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { exint::backend::unchecked_lshr::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_ashr
// CHECK-SAME: (i64 %[[A:.+]], i32 noundef %[[B:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_ashr(a: int, b: u32) -> int {
  // CHECK: %[[C:.+]] = zext nneg i32 %[[B]] to i64
  // CHECK: %[[D:.+]] = ashr i64 %[[A]], %[[C]]
  // CHECK: ret i64 %[[D]]
  unsafe { exint::backend::unchecked_ashr::<_, N>(a, b) }
}

// CHECK-LABEL: define i64 @unchecked_fshl
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshl(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i64
  // CHECK: %[[E:.+]] = tail call i64 @llvm.fshl.i64(i64 %[[A]], i64 %[[B]], i64 %[[D]])
  // CHECK: ret i64 %[[E]]
  unsafe { exint::backend::unchecked_fshl::<_, N>(a, b, c) }
}

// CHECK-LABEL: define i64 @unchecked_fshr
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]], i32 noundef %[[C:.+]])
#[unsafe(no_mangle)]
pub fn unchecked_fshr(a: uint, b: uint, c: u32) -> uint {
  // CHECK: %[[D:.+]] = zext i32 %[[C]] to i64
  // CHECK: %[[E:.+]] = tail call i64 @llvm.fshr.i64(i64 %[[A]], i64 %[[B]], i64 %[[D]])
  // CHECK: ret i64 %[[E]]
  unsafe { exint::backend::unchecked_fshr::<_, N>(a, b, c) }
}

// -----------------------------------------------------------------------------
// Wrapping Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @wrapping_add
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_add(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = add i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  exint::backend::wrapping_add::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @wrapping_sub
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_sub(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = sub i64 %[[A]], %[[B]]
  // CHECK: ret i64 %[[C]]
  exint::backend::wrapping_sub::<_, N>(a, b)
}

// CHECK-LABEL: define i64 @wrapping_mul
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn wrapping_mul(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = mul i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  exint::backend::wrapping_mul::<_, N>(a, b)
}

// -----------------------------------------------------------------------------
// Misc. Operations
// -----------------------------------------------------------------------------

// CHECK-LABEL: define i64 @disjoint_bor
// CHECK-SAME: (i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn disjoint_bor(a: uint, b: uint) -> uint {
  // CHECK: %[[C:.+]] = or disjoint i64 %[[B]], %[[A]]
  // CHECK: ret i64 %[[C]]
  unsafe { exint::backend::disjoint_bor::<_, N>(a, b) }
}
