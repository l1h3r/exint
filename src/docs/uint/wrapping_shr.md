Panic-free bitwise shift-right; yields `self >> mask(rhs)`, where `mask` removes
any high-order bits of `rhs` that would cause the shift to exceed the bitwidth
of the type.

Note that this is *not* the same as a rotate-right; the RHS of a wrapping
shift-right is restricted to the range of the type, rather than the bits shifted
out of the LHS being returned to the other end. The primitive integer types all
implement a [`rotate_right`] function, which may be what you want instead.

[`rotate_right`]: Self::rotate_right

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(128_u24.wrapping_shr(7), 1_u24);
assert_eq!(128_u24.wrapping_shr(96), 128_u24);
# }
```
