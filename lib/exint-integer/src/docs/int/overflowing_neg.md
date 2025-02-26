Negates self, overflowing if this is equal to the minimum value.

Returns a tuple of the negated version of self along with a boolean indicating
whether an overflow happened. If `self` is the minimum value (e.g., `i32::MIN`
for values of type `i32`), then the minimum value will be returned again and
`true` will be returned for an overflow happening.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(2).overflowing_neg(), (int!(-2), false));
assert_eq!(int::MIN.overflowing_neg(), (int::MIN, true));
```
