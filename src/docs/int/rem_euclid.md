Calculates the least nonnegative remainder of `self (mod rhs)`.

This is done as if by the Euclidean division algorithm -- given
`r = self.rem_euclid(rhs)`, the result satisfies
`self = rhs * self.div_euclid(rhs) + r` and `0 <= r < abs(rhs)`.

# Panics

This function will panic if `rhs` is zero or if `self` is `Self::MIN` and `rhs`
is -1. This behavior is not affected by the `overflow-checks` flag.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let a = int!(7);
let b = int!(4);

assert_eq!(a.rem_euclid(b), int!(3));
assert_eq!((-a).rem_euclid(b), int!(1));
assert_eq!(a.rem_euclid(-b), int!(3));
assert_eq!((-a).rem_euclid(-b), int!(1));
```

This will panic:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = int::MIN.rem_euclid(int!(-1));
```
