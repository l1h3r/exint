Returns the bit pattern of `self` reinterpreted as a signed integer of the same size.

This produces the same result as an `as` cast, but ensures that the bit-width
remains the same.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(u24::MAX.cast_signed(), -1_i24);
# }
```
