Strict integer division. Computes `self / rhs`.

Strict division on unsigned types is just normal division. There's no way
overflow could ever happen. This function exists so that all operations are
accounted for in the strict operations.

# Panics

This function will panic if `rhs` is zero.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint!(100).strict_div(uint!(10)), uint!(10));
```

The following panics because of division by zero:

```should_panic
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let _ = uint!(1).strict_div(uint!(0));
```
