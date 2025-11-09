The size of this integer type in bits.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping::<i24>::BITS, i24::BITS);
# }
```
