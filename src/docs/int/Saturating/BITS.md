The size of this integer type in bits.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating::<i24>::BITS, i24::BITS);
# }
```
