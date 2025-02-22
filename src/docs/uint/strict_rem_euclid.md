Strict Euclidean modulo. Computes `self.rem_euclid(rhs)`.

Strict modulo calculation on unsigned types is just the regular remainder
calculation. There's no way overflow could ever happen. This function exists so
that all operations are accounted for in the strict operations. Since, for the
positive integers, all common definitions of division are equal, this is exactly
equal to `self.strict_rem(rhs)`.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(100).strict_rem_euclid(uint!(10)), uint!(0));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = uint!(5).strict_rem_euclid(uint!(0));
```
