Strict integer division. Computes `self / rhs`, panicking if overflow occurred.

# Panics

This function will panic if `rhs` is zero.

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

The only case where such an overflow can occur is when one divides `MIN / -1` on
a signed type (where `MIN` is the negative minimal value for the type); this is
equivalent to `-MIN`, a positive value that is too large to represent in the type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!((int::MIN + int!(1)).strict_div(int!(-1)), int::MAX);
```

The following panics because of overflow:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = int::MIN.strict_div(int!(-1));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = int!(1).strict_div(int!(0));
```
