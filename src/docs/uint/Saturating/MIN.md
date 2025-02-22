The smallest value that can be represented by this integer type.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Saturating;

assert_eq!(<Saturating<uint>>::MIN, Saturating(uint::MIN));
```
