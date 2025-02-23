Computes the absolute value of `self`.

## Overflow behavior

The absolute value of int::MIN cannot be represented as an `int`, and attempting
to calculate it will cause an overflow. This means that code in debug mode will
trigger a panic on this case and optimized code will return `int::MIN` without a
panic.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(10).abs(), int!(10));
assert_eq!(int!(-10).abs(), int!(10));
```
