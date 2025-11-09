The smallest value that can be represented by this integer type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping::<u24>::MIN, Wrapping(u24::MIN));
# }
```
