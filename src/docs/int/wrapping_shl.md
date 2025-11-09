Panic-free bitwise shift-left; yields `self << mask(rhs)`, where `mask` removes
any high-order bits of `rhs` that would cause the shift to exceed the bitwidth
of the type.

Note that this is *not* the same as a rotate-left; the RHS of a wrapping
shift-left is restricted to the range of the type, rather than the bits shifted
out of the LHS being returned to the other end. The primitive integer types all
implement a [`rotate_left`] function, which may be what you want instead.

[`rotate_left`]: Self::rotate_left

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((-1_i24).wrapping_shl(7), -128_i24);
assert_eq!((-1_i24).wrapping_shl(96), -1_i24);
# }
```
