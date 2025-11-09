Computes the absolute value of `self`.

Returns a tuple of the absolute version of self along with a boolean indicating
whether an overflow happened. If self is the minimum value (e.g., `int::MIN` for
values of type int), then the minimum value will be returned again and true will
be returned for an overflow happening.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.overflowing_abs(), (10_i24, false));
assert_eq!((-10_i24).overflowing_abs(), (10_i24, false));
assert_eq!(i24::MIN.overflowing_abs(), (i24::MIN, true));
# }
```
