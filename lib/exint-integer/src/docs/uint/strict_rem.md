Strict integer remainder. Computes `self % rhs`.

Strict remainder calculation on unsigned types is just the regular remainder
calculation. There's no way overflow could ever happen. This function exists so
that all operations are accounted for in the strict operations.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(uint!(100).strict_rem(uint!(10)), uint!(0));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = uint!(5).strict_rem(uint!(0));
```
