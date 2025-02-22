Returns a number representing sign of `self`.

- `0` if the number is zero
- `1` if the number is positive
- `-1` if the number is negative

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(int!(10).signum(), int!(1));
assert_eq!(int!(0).signum(), int!(0));
assert_eq!(int!(-10).signum(), int!(-1));
```
