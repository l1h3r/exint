Calculates `self` &minus; `rhs` &minus; `borrow` and checks for overflow.

Performs "ternary subtraction" by subtracting both an integer operand and a
borrow-in bit from `self`, and returns a tuple of the difference along with a
boolean indicating whether an arithmetic overflow would occur. On overflow, the
wrapped value is returned.

This allows chaining together multiple subtractions to create a wider
subtraction, and can be useful for bignum subtraction. This method should only
be used for the most significant word; for the less significant words the
unsigned method [`uint::borrowing_sub`] should be used.

The output boolean returned by this method is *not* a borrow flag, and should
*not* be subtracted from a more significant word.

If the input borrow is false, this method is equivalent to [`overflowing_sub`].

[`overflowing_sub`]: Self::overflowing_sub

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
// Only the most significant word is signed.
//
//    6    8    (a = 6 × 2^32 + 8)
// - -5    9    (b = -5 × 2^32 + 9)
// ---------
//   10  MAX    (diff = 10 × 2^32 + 2^32 - 1)

let (a1, a0): (int, uint) = (int!(6), uint!(8));
let (b1, b0): (int, uint) = (int!(-5), uint!(9));
let borrow0 = false;

// uint::borrowing_sub for the less significant words
let (diff0, borrow1) = a0.borrowing_sub(b0, borrow0);
assert_eq!(borrow1, true);

// int::borrowing_sub for the most significant word
let (diff1, overflow) = a1.borrowing_sub(b1, borrow1);
assert_eq!(overflow, false);

assert_eq!((diff1, diff0), (int!(10), uint::MAX));
```
