Converts a [`bool`] to [`int`] losslessly.
The resulting value is `0` for `false` and `1` for `true` values.

[`int`]: crate::types::int

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(i24::from(true), 1_i24);
assert_eq!(i24::from(false), 0_i24);
# }
```
