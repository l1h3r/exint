Strict integer addition. Computes `self + rhs`, panicking if overflow occurred.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!((uint::MAX - uint!(2)).strict_add(uint!(1)), uint::MAX - uint!(1));
```

The following panics because of overflow:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = (uint::MAX - uint!(2)).strict_add(uint!(3));
```
