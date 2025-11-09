Checked absolute value. Computes `self.abs()`,
returning `None` if `self == MIN`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(-5_i24.checked_abs(), Some(5_i24));
assert_eq!(i24::MIN.checked_abs(), None);
# }
```
