Returns `true` if `self` is an integer multiple of `rhs`, and false otherwise.

This function is equivalent to `self % rhs == 0`, except that it will not panic
for `rhs == 0`. Instead, `0.is_multiple_of(0) == true`, and for any non-zero
`n`, `n.is_multiple_of(0) == false`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(6_u24.is_multiple_of(2_u24));
assert!(!5_u24.is_multiple_of(2_u24));

assert!(0_u24.is_multiple_of(0_u24));
assert!(!6_u24.is_multiple_of(0_u24));
# }
```
