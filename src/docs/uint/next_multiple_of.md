Calculates the smallest value greater than or equal to `self` that is a multiple of `rhs`.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

On overflow, this function will panic if overflow checks are enabled (default in
debug mode) and wrap if overflow checks are disabled (default in release mode).

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(16_u24.next_multiple_of(8_u24), 16_u24);
assert_eq!(23_u24.next_multiple_of(8_u24), 24_u24);
# }
```
