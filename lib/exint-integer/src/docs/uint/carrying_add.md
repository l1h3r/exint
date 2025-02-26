Calculates `self` + `rhs` + `carry` and returns a tuple containing the sum and the output carry.

Performs "ternary addition" of two integer operands and a carry-in bit, and
returns an output integer and a carry-out bit. This allows chaining together
multiple additions to create a wider addition, and can be useful for bignum
addition.

This can be thought of as a `Self::BITS`-bit "full adder", in the electronics sense.

If the input carry is false, this method is equivalent to [`overflowing_add`],
and the output carry is equal to the overflow flag. Note that although carry and
overflow flags are similar for unsigned integers, they are different for signed
integers.

[`overflowing_add`]: Self::overflowing_add

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
//    3  MAX    (a = 3 × 2^32 + 2^32 - 1)
// +  5    7    (b = 5 × 2^32 + 7)
// ---------
//    9    6    (sum = 9 × 2^32 + 6)

let (a1, a0): (uint, uint) = (uint!(3), uint::MAX);
let (b1, b0): (uint, uint) = (uint!(5), uint!(7));
let carry0 = false;

let (sum0, carry1) = a0.carrying_add(b0, carry0);
assert_eq!(carry1, true);
let (sum1, carry2) = a1.carrying_add(b1, carry1);
assert_eq!(carry2, false);

assert_eq!((sum1, sum0), (uint!(9), uint!(6)));
```
