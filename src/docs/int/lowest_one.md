Returns the index of the lowest bit set to one in `self`, or `None` if `self` is `0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(0x0_i24.lowest_one(), None);
assert_eq!(0x1_i24.lowest_one(), Some(0));
assert_eq!(0x10_i24.lowest_one(), Some(4));
assert_eq!(0x1f_i24.lowest_one(), Some(0));
# }
```
