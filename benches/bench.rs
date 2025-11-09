#![feature(random)]
#![allow(non_camel_case_types)]

use ::core::hint::black_box;
use ::core::random::Distribution;
use ::criterion::BatchSize;
use ::criterion::BenchmarkGroup;
use ::criterion::Criterion;
use ::criterion::criterion_group;
use ::criterion::criterion_main;
use ::criterion::measurement::WallTime;
use ::exint_benchmarks::BenchRng;

const N: usize = 32;

const ZERO: ::exint::uint::<N> = ::exint::uint::<N>::from_u8(0);
const ONE: ::exint::uint::<N> = ::exint::uint::<N>::from_u8(1);

fn bench_rng() -> BenchRng {
  const SEED: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
  BenchRng::from_seed(SEED)
}

fn uint(rng: &mut BenchRng) -> ::exint::uint::<N> {
  (..).sample(rng)
}

fn int(rng: &mut BenchRng) -> ::exint::int::<N> {
  (..).sample(rng)
}

fn u32(rng: &mut BenchRng) -> u32 {
  (..).sample(rng)
}

fn uint_nz(rng: &mut BenchRng) -> ::exint::uint::<N> {
  loop {
    let this: ::exint::uint::<N> = uint(rng);

    if this != ZERO {
      return this;
    }
  }
}

macro_rules! op {
  ($group:ident, $name:literal, |$bencher:ident| {
    args = [$($setup:expr),* $(,)?] $(,)?
    call = |$($binding:ident),+| -> $expr:expr $(,)?
  }) => {
    $group.bench_function($name, |$bencher| {
      $bencher.iter_batched(
        || ($($setup,)*),
        |($($binding,)+)| black_box($expr),
        BatchSize::SmallInput,
      )
    })
  };
}

fn bench_bitwise(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("bitwise");

  op!(group, "bitand", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a & b
  });

  op!(group, "bitor", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a | b
  });

  op!(group, "bitxor", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a ^ b
  });

  op!(group, "not", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> !a
  });

  group.finish();
}

fn bench_compare(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("compare");

  op!(group, "eq random", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a == b
  });

  group.finish();
}

fn bench_inspect(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("inspect");

  op!(group, "ctpop", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.count_ones()
  });

  op!(group, "ctlz", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.leading_zeros()
  });

  op!(group, "cttz", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.trailing_zeros()
  });

  group.finish();
}

fn bench_add(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("add");

  op!(group, "checked_add", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.checked_add(b)
  });

  op!(group, "overflowing_add", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.overflowing_add(b)
  });

  op!(group, "saturating_add", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.saturating_add(b)
  });

  op!(group, "wrapping_add", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.wrapping_add(b)
  });

  group.finish();
}

fn bench_add_signed(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("add_signed");

  op!(group, "checked_add_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.checked_add_signed(b)
  });

  op!(group, "overflowing_add_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.overflowing_add_signed(b)
  });

  op!(group, "saturating_add_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.saturating_add_signed(b)
  });

  op!(group, "wrapping_add_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.wrapping_add_signed(b)
  });

  group.finish();
}

fn bench_sub(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("sub");

  op!(group, "checked_sub", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.checked_sub(b)
  });

  op!(group, "overflowing_sub", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.overflowing_sub(b)
  });

  op!(group, "saturating_sub", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.saturating_sub(b)
  });

  op!(group, "wrapping_sub", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.wrapping_sub(b)
  });

  group.finish();
}

fn bench_sub_signed(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("sub_signed");

  op!(group, "checked_sub_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.checked_sub_signed(b)
  });

  op!(group, "overflowing_sub_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.overflowing_sub_signed(b)
  });

  op!(group, "saturating_sub_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.saturating_sub_signed(b)
  });

  op!(group, "wrapping_sub_signed", |bencher| {
    args = [uint(&mut rng), int(&mut rng)]
    call = |a, b| -> a.wrapping_sub_signed(b)
  });

  group.finish();
}

fn bench_mul(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("mul");

  op!(group, "checked_mul", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.checked_mul(b)
  });

  op!(group, "overflowing_mul", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.overflowing_mul(b)
  });

  op!(group, "saturating_mul", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.saturating_mul(b)
  });

  op!(group, "wrapping_mul", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.wrapping_mul(b)
  });

  group.finish();
}

fn bench_div(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("div");

  op!(group, "checked_div", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.checked_div(b)
  });

  op!(group, "overflowing_div", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.overflowing_div(b)
  });

  op!(group, "saturating_div", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.saturating_div(b)
  });

  op!(group, "wrapping_div", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.wrapping_div(b)
  });

  group.finish();
}

fn bench_rem(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("rem");

  op!(group, "checked_rem", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.checked_rem(b)
  });

  op!(group, "overflowing_rem", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.overflowing_rem(b)
  });

  op!(group, "wrapping_rem", |bencher| {
    args = [uint(&mut rng), uint_nz(&mut rng)]
    call = |a, b| -> a.wrapping_rem(b)
  });

  group.finish();
}

fn bench_shl(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("shl");

  op!(group, "checked_shl", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.checked_shl(b)
  });

  op!(group, "overflowing_shl", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.overflowing_shl(b)
  });

  op!(group, "unbounded_shl", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.unbounded_shl(b)
  });

  op!(group, "wrapping_shl", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.wrapping_shl(b)
  });

  group.finish();
}

fn bench_shr(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("shr");

  op!(group, "checked_shr", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.checked_shr(b)
  });

  op!(group, "overflowing_shr", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.overflowing_shr(b)
  });

  op!(group, "unbounded_shr", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.unbounded_shr(b)
  });

  op!(group, "wrapping_shr", |bencher| {
    args = [ONE, u32(&mut rng)]
    call = |a, b| -> a.wrapping_shr(b)
  });

  group.finish();
}

fn bench_ilog(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("ilog");

  op!(group, "checked_ilog", |bencher| {
    args = [uint(&mut rng), uint(&mut rng)]
    call = |a, b| -> a.checked_ilog(b)
  });

  op!(group, "checked_ilog2", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.checked_ilog2()
  });

  op!(group, "checked_ilog10", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.checked_ilog10()
  });

  group.finish();
}

fn bench_pow_static(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("pow_static");

  op!(group, "checked_pow", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.checked_pow(3)
  });

  op!(group, "overflowing_pow", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.overflowing_pow(3)
  });

  op!(group, "saturating_pow", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.saturating_pow(3)
  });

  op!(group, "wrapping_pow", |bencher| {
    args = [uint(&mut rng)]
    call = |a| -> a.wrapping_pow(3)
  });

  group.finish();
}

fn bench_pow_random(c: &mut Criterion) {
  let mut rng: BenchRng = bench_rng();
  let mut group: BenchmarkGroup<'_, WallTime> = c.benchmark_group("pow_random");

  op!(group, "checked_pow", |bencher| {
    args = [uint(&mut rng), u32(&mut rng)]
    call = |a, b| -> a.checked_pow(b)
  });

  op!(group, "overflowing_pow", |bencher| {
    args = [uint(&mut rng), u32(&mut rng)]
    call = |a, b| -> a.overflowing_pow(b)
  });

  op!(group, "saturating_pow", |bencher| {
    args = [uint(&mut rng), u32(&mut rng)]
    call = |a, b| -> a.saturating_pow(b)
  });

  op!(group, "wrapping_pow", |bencher| {
    args = [uint(&mut rng), u32(&mut rng)]
    call = |a, b| -> a.wrapping_pow(b)
  });

  group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default();
  targets =
    bench_bitwise,
    bench_compare,
    bench_inspect,
    bench_add, bench_add_signed,
    bench_sub, bench_sub_signed,
    bench_mul, bench_div, bench_rem,
    bench_shl, bench_shr,
    bench_ilog, bench_pow_static, bench_pow_random,
}

criterion_main!(benches);
