Returns the number of leading zeros in the binary representation of `self`.

Depending on what you're doing with the value, you might also be interested in the
[`ilog2`] function which returns a consistent number, even if the type widens.

[`ilog2`]: Self::ilog2

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!((u24::MAX >> 2u32).leading_zeros(), 2);
assert_eq!(0_u24.leading_zeros(), 24);
assert_eq!(u24::MAX.leading_zeros(), 0);
# }
```
