use exint::uint;

// CHECK-LABEL: define range(i16 0, 512) i16 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> (uint<1>, bool) {
  // CHECK: %[[C:.+]] = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %[[A]], i8 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i8, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i8, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i16 256, i16 0
  // CHECK: %[[G:.+]] = zext i8 %[[D]] to i16
  // CHECK: %[[H:.+]] = or disjoint i16 %[[F]], %[[G]]
  // CHECK: ret i16 %[[H]]
  a.overflowing_mul(b)
}

// CHECK-LABEL: define range(i24 0, 131072) i24 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> (uint<2>, bool) {
  // CHECK: %[[C:.+]] = tail call { i16, i1 } @llvm.umul.with.overflow.i16(i16 %[[A]], i16 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i16, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i16, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i24 65536, i24 0
  // CHECK: %[[G:.+]] = zext i16 %[[D]] to i24
  // CHECK: %[[H:.+]] = or disjoint i24 %[[F]], %[[G]]
  // CHECK: ret i24 %[[H]]
  a.overflowing_mul(b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> (uint<4>, bool) {
  // CHECK: %[[C:.+]] = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %[[A]], i32 %[[B]])
  // CHECK: %[[D:.+]] = extractvalue { i32, i1 } %[[C]], 0
  // CHECK: %[[E:.+]] = extractvalue { i32, i1 } %[[C]], 1
  // CHECK: %[[F:.+]] = select i1 %[[E]], i40 4294967296, i40 0
  // CHECK: %[[G:.+]] = zext i32 %[[D]] to i40
  // CHECK: %[[H:.+]] = or disjoint i40 %[[F]], %[[G]]
  // CHECK: ret i40 %[[H]]
  a.overflowing_mul(b)
}

// CHECK-LABEL: define void @uint_8
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> (uint<8>, bool) {
  // CHECK: %[[D:.+]] = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %[[A]], i64 %[[B]])
  // CHECK: %[[E:.+]] = extractvalue { i64, i1 } %[[D]], 0
  // CHECK: %[[F:.+]] = extractvalue { i64, i1 } %[[D]], 1
  // CHECK: store i64 %[[E]], ptr %[[C]], align 1
  // CHECK: %[[G:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[H:.+]] = zext i1 %[[F]] to i8
  // CHECK: store i8 %[[H]], ptr %[[G]], align 1
  // CHECK: ret void
  a.overflowing_mul(b)
}

// CHECK-LABEL: define void @uint_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> (uint<16>, bool) {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = tail call { i128, i1 } @llvm.umul.with.overflow.i128(i128 %[[D]], i128 %[[E]])
  // CHECK: %[[G:.+]] = extractvalue { i128, i1 } %[[F]], 0
  // CHECK: %[[H:.+]] = extractvalue { i128, i1 } %[[F]], 1
  // CHECK: store i128 %[[G]], ptr %[[C]], align 1
  // CHECK: %[[I:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[J:.+]] = zext i1 %[[H]] to i8
  // CHECK: store i8 %[[J]], ptr %[[I]], align 1
  // CHECK: ret void
  a.overflowing_mul(b)
}
