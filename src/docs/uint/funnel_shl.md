Performs a left funnel shift (concatenates `self` and `rhs`, with `self`
making up the most significant half, then shifts the combined value left
by `bits`, and most significant half is extracted to produce the result).

Please note this isn't the same operation as the `<<` shifting operator or
[`rotate_left`], although `a.funnel_shl(a, n)` is *equivalent* to `a.rotate_left(n)`.

# Panics

This function will panic if `bits` is greater than or equal to the number of bits in `self`

[`rotate_left`]: Self::rotate_left

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 0x120034_u24;
let b = 0xAABBCC_u24;
let m = 0x0034AA_u24;

assert_eq!(a.funnel_shl(b, 8), m);
# }
```
