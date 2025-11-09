The largest value that can be represented by this integer type.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

assert_eq!(Wrapping::<u24>::MAX, Wrapping(u24::MAX));
# }
```
