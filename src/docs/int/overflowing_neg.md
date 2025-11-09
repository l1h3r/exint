Negates self, overflowing if this is equal to the minimum value.

Returns a tuple of the negated version of self along with a boolean indicating
whether an overflow happened. If `self` is the minimum value (e.g., `i32::MIN`
for values of type `i32`), then the minimum value will be returned again and
`true` will be returned for an overflow happening.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(2_i24.overflowing_neg(), (-2_i24, false));
assert_eq!(i24::MIN.overflowing_neg(), (i24::MIN, true));
# }
```
