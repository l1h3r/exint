# exint - Fixed-Width Integer Types for Rust

[![github][shield-img-github]][shield-url-github]
[![crates.io][shield-img-crates]][shield-url-crates]
[![docs.rs][shield-img-docs]][shield-url-docs]
[![build status][shield-img-ci]][shield-url-ci]

A `no_std` Rust library providing stack-allocated integer types of **any** byte width (`uint<3>`, `int<5>`, `uint<10>`, and so on) alongside familiar aliases like `u24`, `i40`, and `u80`.

Standard Rust gives you `u8` through `u128`. exint fills every gap in between and beyond with types that behave like built-in integers: they implement the same traits, support the same operations, and work in `const` contexts.

### Why exint?

Protocols, file formats, and hardware registers rarely align to powers of two. Rather than masking a `u32` and hoping every call site remembers, `uint<3>` makes the 24-bit width part of the type system.

Under the hood, exint maps each width to an optimized code path: standard sizes (`1|2|4|8|16` bytes) use native primitives directly, non-standard sizes (`3|5|6|7|9..15` bytes) widen to the next primitive with LLVM-recognized narrowing patterns, and arbitrary sizes fall back to portable limb-based algorithms. The result is that `u24::wrapping_add` compiles to an `add i24` in LLVM IR, not a `u32` add with a manual mask.

## Quick Start

```rust
use exint::primitive::u24;

fn main() {
  let one: u24 = u24::from(1_u8);
  let two: u24 = u24::from(2_u8);

  assert_eq!(u24::MIN, u24::MAX.wrapping_add(one));
  assert_eq!(u24::MAX, u24::try_from(16777215_u32).unwrap());
  assert_eq!(u24::MAX / two, u24::MAX >> 1_u32);
}
```

## Features

- Signed (`int<N>`) and unsigned (`uint<N>`) types for any byte width
- Convenient aliases: `u24`, `u40`, `i48`, `u80`, ...
- `no_std` compatible, zero dependencies
- Fully `const`-evaluable
- Compile-time specialization targeting ideal LLVM IR per width
- Literal macro support (`exint::uint!`, `exint::uint_strict!`)

## Literals

`exint` provides a simple procedural macro to simplify working with literal values.

The above example can be re-written as:

```rust
use exint::primitive::u24;

fn main() {
  exint::uint! {
    assert_eq!(u24::MIN, u24::MAX.wrapping_add(1_u24));
    assert_eq!(u24::MAX, 16777215_u24);
    assert_eq!(u24::MAX / 2_u24, u24::MAX >> 1_u32);
  }
}
```

Use the `exint::uint_strict!` macro to avoid converting the core integer types.

```rust
use exint::uint;

fn main() {
  exint::uint! {
    let a: uint<3> = 123_u24;  // <-- Converted
    let b: uint<4> = 456_u32;  // <-- Converted
  }

  exint::uint_strict! {
    let a: uint<3> = 123_u24;  // <-- Converted
    let b: u32 = 456_u32;  // <-- Not converted
  }
}
```

## Security

This crate is **not** intended for cryptographic use. Consider using [`crypto-bigint`] if you need an integer type suitable for cryptographic applications.

#### License

<sup>
  Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
  Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>

[//]: # (links)

[`crypto-bigint`]: https://crates.io/crates/crypto-bigint

[//]: # (badges)

[shield-url-github]: https://github.com/l1h3r/exint
[shield-img-github]: https://img.shields.io/badge/github-l1h3r/exint-main?style=flat-square&logo=github

[shield-url-crates]: https://crates.io/crates/exint
[shield-img-crates]: https://img.shields.io/crates/v/exint?style=flat-square&logo=rust

[shield-url-docs]: https://docs.rs/exint
[shield-img-docs]: https://img.shields.io/docsrs/exint?style=flat-square&logo=docs.rs

[shield-url-ci]: https://github.com/l1h3r/exint/actions/workflows/ci.yml?query=branch:main
[shield-img-ci]: https://img.shields.io/github/actions/workflow/status/l1h3r/exint/ci.yml?style=flat-square
