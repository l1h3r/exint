Strict negation. Computes `-self`, panicking unless `self == 0`.

Note that negating any positive integer will overflow.

# Panics

## Overflow behavior

This function will always panic on overflow, regardless of whether overflow checks are enabled.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(0).strict_neg(), uint!(0));
```

The following panics because of overflow:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = uint!(1).strict_neg();
```
