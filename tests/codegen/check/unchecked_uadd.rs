use exint::uint;

// CHECK-LABEL: define i8 @uint_1
#[unsafe(no_mangle)]
pub fn uint_1(a: uint<1>, b: uint<1>) -> uint<1> {
  // CHECK: %[[reg:.*]] = add nuw i8 %{{.*}}, %{{.*}}
  // CHECK: ret i8 %[[reg]]
  unsafe { a.unchecked_add(b) }
}

// CHECK-LABEL: define i16 @uint_2
#[unsafe(no_mangle)]
pub fn uint_2(a: uint<2>, b: uint<2>) -> uint<2> {
  // CHECK: %[[reg:.*]] = add nuw i16 %{{.*}}, %{{.*}}
  // CHECK: ret i16 %[[reg]]
  unsafe { a.unchecked_add(b) }
}

// CHECK-LABEL: define i32 @uint_4
#[unsafe(no_mangle)]
pub fn uint_4(a: uint<4>, b: uint<4>) -> uint<4> {
  // CHECK: %[[reg:.*]] = add nuw i32 %{{.*}}, %{{.*}}
  // CHECK: ret i32 %[[reg]]
  unsafe { a.unchecked_add(b) }
}

// CHECK-LABEL: define i64 @uint_8
#[unsafe(no_mangle)]
pub fn uint_8(a: uint<8>, b: uint<8>) -> uint<8> {
  // CHECK: %[[reg:.*]] = add nuw i64 %{{.*}}, %{{.*}}
  // CHECK: ret i64 %[[reg]]
  unsafe { a.unchecked_add(b) }
}

// CHECK-LABEL: define void @uint_16
#[unsafe(no_mangle)]
pub fn uint_16(a: uint<16>, b: uint<16>) -> uint<16> {
  // CHECK: %[[reg_a:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_b:.*]] = load i128, ptr %{{.*}}, align 1
  // CHECK: %[[reg_c:.*]] = add nuw i128 %[[reg_b]], %[[reg_a]]
  // CHECK: store i128 %[[reg_c]], ptr %{{.*}}, align 1
  // CHECK: ret void
  unsafe { a.unchecked_add(b) }
}
