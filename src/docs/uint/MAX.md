The largest value that can be represented by this integer type. (2<sup>32</sup> &minus; 1).

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint::MAX, uint!(4294967295));
```
