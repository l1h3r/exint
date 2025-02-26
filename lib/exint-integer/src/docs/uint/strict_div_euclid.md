Strict Euclidean division. Computes `self.div_euclid(rhs)`.

Strict division on unsigned types is just normal division. There's no way
overflow could ever happen. This function exists so that all operations are
accounted for in the strict operations. Since, for the positive integers, all
common definitions of division are equal, this is exactly equal to
`self.strict_div(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(100).strict_div_euclid(uint!(10)), uint!(10));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = uint!(1).strict_div_euclid(uint!(0));
```
