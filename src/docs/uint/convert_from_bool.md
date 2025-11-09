Converts a [`bool`] to [`uint`] losslessly.
The resulting value is `0` for `false` and `1` for `true` values.

[`uint`]: crate::types::uint

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(u24::from(true), 1_u24);
assert_eq!(u24::from(false), 0_u24);
# }
```
