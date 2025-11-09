# Exint - Exotic Integer Types

[![github][shield-img-github]][shield-url-github]
[![crates.io][shield-img-crates]][shield-url-crates]
[![docs.rs][shield-img-docs]][shield-url-docs]
[![build status][shield-img-ci]][shield-url-ci]

A pure Rust implementation of generic signed and unsigned integers.

This crate provides the `no_std`-compatible types `int<N>` and `uint<N>`, which mimic the behavior and API of the core integer primitives while supporting fixed-width values of arbitrary precision.

```toml
[dependencies]
exint = "0.1.0"
```

## Basic example

```rust
use exint::primitive::u24;

fn main() {
  let a: u24 = u24::from(123_u16);
  let b: u24 = u24::from(456_u16);
  let c: u24 = u24::from(10_u16);

  println!("d = {}", a + b);
  println!("e = {}", a.wrapping_sub(b));
  println!("f = {}", u24::MAX - (c >> 1_u32));
}
```

## Literals

Since there is no way to define your own literal format in Rust, `exint` provides a simple procedural macro.

The above example can be re-written as the following:

```rust
use exint::primitive::u24;

fn main() {
  exint::uint! {
    let a: u24 = 123_u24;
    let b: u24 = 456_u24;
    let c: u24 = 10_u24;

    println!("d = {}", a + b);
    println!("e = {}", a.wrapping_sub(b));
    println!("f = {}", u24::MAX - (c >> 1_u32));
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

## Features

| Group          | Name                                 | Nightly        |
| :------------- | :----------------------------------- | :------------: |
|                | `std`                                | no             |
| `all_unstable` |                                      |                |
|                | `exact_bitshifts`                    | no             |
|                | `int_from_ascii`                     | no             |
|                | `int_lowest_highest_one`             | no             |
|                | `int_roundings`                      | no             |
|                | `is_ascii_octdigit`                  | no             |
|                | `isolate_most_least_significant_one` | no             |
|                | `uint_bit_width`                     | no             |
|                | `unchecked_neg`                      | no             |
|                | `unchecked_shifts`                   | no             |
|                | `utf16_extra`                        | no             |
|                | `wrapping_next_power_of_two`         | no             |
| `all_nightly`  |                                      |                |
|                | `adt_const_params`                   | yes            |
|                | `ascii_char`                         | yes            |
|                | `bigint_helper_methods`              | yes            |
|                | `disjoint_bitor`                     | yes            |
|                | `exact_div`                          | yes            |
|                | `f16`                                | yes            |
|                | `f128`                               | yes            |
|                | `funnel_shifts`                      | yes            |
|                | `integer_atomics`                    | yes            |
|                | `never_type`                         | yes            |
|                | `random`                             | yes            |
|                | `step_trait`                         | yes            |
|                | `structural_match`                   | yes            |
|                | `trusted_step`                       | yes            |
|                | `unsized_const_params`               | yes            |
| `all_const`    |                                      |                |
|                | `const_traits`                       | yes            |
|                | `const_clone`                        | yes            |
|                | `const_cmp`                          | yes            |
|                | `const_convert`                      | yes            |
|                | `const_default`                      | yes            |
|                | `const_ops`                          | yes            |
|                | `const_option`                       | yes            |
|                | `const_result`                       | yes            |
| `all_backend`  |                                      |                |
|                | `core_intrinsics`                    | yes            |
|                | `const_eval_select`                  | yes            |
|                | `min_specialization`                 | yes            |
|                | `portable_simd`                      | yes            |
|                | `proc_macro_diagnostic`              | yes            |

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
