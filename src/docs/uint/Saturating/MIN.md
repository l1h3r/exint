The smallest value that can be represented by this integer type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

assert_eq!(Saturating::<u24>::MIN, Saturating(u24::MIN));
# }
```
