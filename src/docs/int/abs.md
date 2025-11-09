Computes the absolute value of `self`.

## Overflow behavior

The absolute value of `int::MIN` cannot be represented as an `int`, and attempting
to calculate it will cause an overflow. This means that code in debug mode will
trigger a panic on this case and optimized code will return `int::MIN` without a
panic.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(10_i24.abs(), 10_i24);
assert_eq!((-10_i24).abs(), 10_i24);
# }
```
