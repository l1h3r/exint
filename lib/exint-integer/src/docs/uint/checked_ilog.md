Returns the logarithm of the number with respect to an arbitrary base, rounded down.

Returns `None` if the number is zero, or if the base is not at least 2.

This method might not be optimized owing to implementation details;
`checked_ilog2` can produce results more efficiently for base 2, and
`checked_ilog10` can produce results more efficiently for base 10.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).checked_ilog(uint!(5)), Some(1));
```
