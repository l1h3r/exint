Checked Euclidean modulo. Computes `self.rem_euclid(rhs)`,
returning `None` if `rhs == 0`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(5).checked_rem_euclid(uint!(2)), Some(uint!(1)));
assert_eq!(uint!(5).checked_rem_euclid(uint!(0)), None);
```
