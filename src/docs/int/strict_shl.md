Strict shift left. Computes `self << rhs`,
panicking if `rhs` is larger than or equal to the number of bits in `self`.

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
assert_eq!(int!(0x1).strict_shl(4), int!(0x10));
```

The following panics because of overflow:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = int!(0x1).strict_shl(129);
```
