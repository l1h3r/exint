Returns the base 2 logarithm of the number, rounded down.

Returns `None` if the number is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(2).checked_ilog2(), Some(1));
```
