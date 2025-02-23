Checked Euclidean division. Computes `self.div_euclid(rhs)`,
returning `None` if `rhs == 0`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(128).checked_div_euclid(uint!(2)), Some(uint!(64)));
assert_eq!(uint!(1).checked_div_euclid(uint!(0)), None);
```
