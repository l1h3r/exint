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
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(1).wrapping_shl(7), uint!(128));
assert_eq!(uint!(1).wrapping_shl(128), uint!(1));
```
