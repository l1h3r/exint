Performs a right funnel shift (concatenates `self` and `rhs`, with `self`
making up the most significant half, then shifts the combined value right
by `bits`, and least significant half is extracted to produce the result).

Please note this isn't the same operation as the `>>` shifting operator or
[`rotate_right`], although `a.funnel_shr(a, n)` is *equivalent* to `a.rotate_right(n)`.

# Panics

This function will panic if `bits` is greater than or equal to the number of bits in `self`

[`rotate_right`]: Self::rotate_right

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let a = 0x120034_u24;
let b = 0xAABBCC_u24;
let m = 0x34AABB_u24;

assert_eq!(a.funnel_shr(b, 8), m);
# }
```
