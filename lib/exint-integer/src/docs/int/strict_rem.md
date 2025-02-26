Strict integer remainder. Computes `self % rhs`, panicking if the division results in overflow.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

The only case where such an overflow can occur is `x % y` for `MIN / -1` on a
signed type (where `MIN` is the negative minimal value), which is invalid due to
implementation artifacts.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int!(5).strict_rem(int!(2)), int!(1));
```

The following panics because of overflow:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = int::MIN.strict_rem(int!(-1));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let _ = int!(5).strict_rem(int!(0));
```
