Computes the absolute value of `self`.

Returns a tuple of the absolute version of self along with a boolean indicating
whether an overflow happened. If self is the minimum value (e.g., int::MIN for
values of type int), then the minimum value will be returned again and true will
be returned for an overflow happening.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(10).overflowing_abs(), (int!(10), false));
assert_eq!(int!(-10).overflowing_abs(), (int!(10), false));
assert_eq!(int::MIN.overflowing_abs(), (int::MIN, true));
```
