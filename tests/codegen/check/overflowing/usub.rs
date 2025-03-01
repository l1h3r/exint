use exint::uint;

// CHECK-LABEL: define range(i16 0, 512) i16 @uint_1
// CHECK-SAME: (i8 %[[A:.+]], i8 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> (uint<1>, bool) {
  // CHECK: %[[C:.+]] = sub i8 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i8 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i16 256, i16 0
  // CHECK: %[[F:.+]] = zext i8 %[[C]] to i16
  // CHECK: %[[G:.+]] = or disjoint i16 %[[E]], %[[F]]
  // CHECK: ret i16 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i24 0, 131072) i24 @uint_2
// CHECK-SAME: (i16 %[[A:.+]], i16 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> (uint<2>, bool) {
  // CHECK: %[[C:.+]] = sub i16 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i16 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i24 65536, i24 0
  // CHECK: %[[F:.+]] = zext i16 %[[C]] to i24
  // CHECK: %[[G:.+]] = or disjoint i24 %[[E]], %[[F]]
  // CHECK: ret i24 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i32 0, 33554432) i32 @uint_3
// CHECK-SAME: (i24 %[[A:.+]], i24 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_3(a: uint<3>, b: uint<3>) -> (uint<3>, bool) {
  // CHECK-DAG: %[[C:.+]] = sub i24 %[[A]], %[[B]]
  // CHECK-DAG: %[[D:.+]] = icmp ult i24 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i32 16777216, i32 0
  // CHECK: %[[F:.+]] = zext i24 %[[C]] to i32
  // CHECK: %[[G:.+]] = or disjoint i32 %[[E]], %[[F]]
  // CHECK: ret i32 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i40 0, 8589934592) i40 @uint_4
// CHECK-SAME: (i32 %[[A:.+]], i32 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> (uint<4>, bool) {
  // CHECK: %[[C:.+]] = sub i32 %[[A]], %[[B]]
  // CHECK: %[[D:.+]] = icmp ult i32 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i40 4294967296, i40 0
  // CHECK: %[[F:.+]] = zext i32 %[[C]] to i40
  // CHECK: %[[G:.+]] = or disjoint i40 %[[E]], %[[F]]
  // CHECK: ret i40 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i48 0, 2199023255552) i48 @uint_5
// CHECK-SAME: (i40 %[[A:.+]], i40 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_5(a: uint<5>, b: uint<5>) -> (uint<5>, bool) {
  // CHECK-DAG: %[[C:.+]] = sub i40 %[[A]], %[[B]]
  // CHECK-DAG: %[[D:.+]] = icmp ult i40 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i48 1099511627776, i48 0
  // CHECK: %[[F:.+]] = zext i40 %[[C]] to i48
  // CHECK: %[[G:.+]] = or disjoint i48 %[[E]], %[[F]]
  // CHECK: ret i48 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i56 0, 562949953421312) i56 @uint_6
// CHECK-SAME: (i48 %[[A:.+]], i48 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_6(a: uint<6>, b: uint<6>) -> (uint<6>, bool) {
  // CHECK-DAG: %[[C:.+]] = sub i48 %[[A]], %[[B]]
  // CHECK-DAG: %[[D:.+]] = icmp ult i48 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i56 281474976710656, i56 0
  // CHECK: %[[F:.+]] = zext i48 %[[C]] to i56
  // CHECK: %[[G:.+]] = or disjoint i56 %[[E]], %[[F]]
  // CHECK: ret i56 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define range(i64 0, 144115188075855872) i64 @uint_7
// CHECK-SAME: (i56 %[[A:.+]], i56 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_7(a: uint<7>, b: uint<7>) -> (uint<7>, bool) {
  // CHECK-DAG: %[[C:.+]] = sub i56 %[[A]], %[[B]]
  // CHECK-DAG: %[[D:.+]] = icmp ult i56 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = select i1 %[[D]], i64 72057594037927936, i64 0
  // CHECK: %[[F:.+]] = zext i56 %[[C]] to i64
  // CHECK: %[[G:.+]] = or disjoint i64 %[[E]], %[[F]]
  // CHECK: ret i64 %[[G]]
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_8
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], i64 %[[A:.+]], i64 %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> (uint<8>, bool) {
  // CHECK: %[[D:.+]] = sub i64 %[[A]], %[[B]]
  // CHECK: %[[E:.+]] = icmp ult i64 %[[A]], %[[B]]
  // CHECK: store i64 %[[D]], ptr %[[C]], align 1
  // CHECK: %[[F:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 8
  // CHECK: %[[G:.+]] = zext i1 %[[E]] to i8
  // CHECK: store i8 %[[G]], ptr %[[F]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_9
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_9(a: uint<9>, b: uint<9>) -> (uint<9>, bool) {
  // CHECK: %[[D:.+]] = load i72, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i72, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i72 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i72 %[[D]], %[[E]]
  // CHECK: store i72 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 9
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_10
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_10(a: uint<10>, b: uint<10>) -> (uint<10>, bool) {
  // CHECK: %[[D:.+]] = load i80, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i80, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i80 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i80 %[[D]], %[[E]]
  // CHECK: store i80 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 10
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_11
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_11(a: uint<11>, b: uint<11>) -> (uint<11>, bool) {
  // CHECK: %[[D:.+]] = load i88, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i88, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i88 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i88 %[[D]], %[[E]]
  // CHECK: store i88 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 11
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_12
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_12(a: uint<12>, b: uint<12>) -> (uint<12>, bool) {
  // CHECK: %[[D:.+]] = load i96, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i96, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i96 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i96 %[[D]], %[[E]]
  // CHECK: store i96 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 12
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_13
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_13(a: uint<13>, b: uint<13>) -> (uint<13>, bool) {
  // CHECK: %[[D:.+]] = load i104, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i104, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i104 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i104 %[[D]], %[[E]]
  // CHECK: store i104 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 13
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_14
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_14(a: uint<14>, b: uint<14>) -> (uint<14>, bool) {
  // CHECK: %[[D:.+]] = load i112, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i112, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i112 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i112 %[[D]], %[[E]]
  // CHECK: store i112 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 14
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_15
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_15(a: uint<15>, b: uint<15>) -> (uint<15>, bool) {
  // CHECK: %[[D:.+]] = load i120, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i120, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i120 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i120 %[[D]], %[[E]]
  // CHECK: store i120 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 15
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}

// CHECK-LABEL: define void @uint_16
// CHECK-SAME: (ptr {{.*}} %[[C:.+]], ptr {{.*}} %[[A:.+]], ptr {{.*}} %[[B:.+]])
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> (uint<16>, bool) {
  // CHECK: %[[D:.+]] = load i128, ptr %[[A]], align 1
  // CHECK: %[[E:.+]] = load i128, ptr %[[B]], align 1
  // CHECK: %[[F:.+]] = sub i128 %[[D]], %[[E]]
  // CHECK: %[[G:.+]] = icmp ult i128 %[[D]], %[[E]]
  // CHECK: store i128 %[[F]], ptr %[[C]], align 1
  // CHECK: %[[H:.+]] = getelementptr inbounds nuw i8, ptr %[[C]], i64 16
  // CHECK: %[[I:.+]] = zext i1 %[[G]] to i8
  // CHECK: store i8 %[[I]], ptr %[[H]], align 1
  // CHECK: ret void
  a.overflowing_sub(b)
}
