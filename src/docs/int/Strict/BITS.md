The size of this integer type in bits.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Strict;

assert_eq!(<Strict<int>>::BITS, int::BITS);
```
