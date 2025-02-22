Calculates `self` &minus; `rhs` &minus; `borrow` and returns a tuple containing
the difference and the output borrow.

Performs "ternary subtraction" by subtracting both an integer operand and a
borrow-in bit from `self`, and returns an output integer and a borrow-out bit.
This allows chaining together multiple subtractions to create a wider
subtraction, and can be useful for bignum subtraction.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
//    9    6    (a = 9 × 2^$bits + 6)
// -  5    7    (b = 5 × 2^$bits + 7)
// ---------
//    3  MAX    (diff = 3 × 2^$bits + 2^$bits - 1)

let (a1, a0): (uint, uint) = (uint!(9), uint!(6));
let (b1, b0): (uint, uint) = (uint!(5), uint!(7));
let borrow0 = false;

let (diff0, borrow1) = a0.borrowing_sub(b0, borrow0);
assert_eq!(borrow1, true);
let (diff1, borrow2) = a1.borrowing_sub(b1, borrow1);
assert_eq!(borrow2, false);

assert_eq!((diff1, diff0), (uint!(3), uint::MAX));
```
