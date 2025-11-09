Calculates `self` + `rhs` + `carry` and checks for overflow.

Performs "ternary addition" of two integer operands and a carry-in bit, and
returns a tuple of the sum along with a boolean indicating whether an arithmetic
overflow would occur. On overflow, the wrapped value is returned.

This allows chaining together multiple additions to create a wider addition, and
can be useful for bignum addition. This method should only be used for the most
significant word; for the less significant words the unsigned method
[`uint::carrying_add`] should be used.

The output boolean returned by this method is *not* a carry flag, and should
*not* be added to a more significant word.

If the input carry is false, this method is equivalent to [`overflowing_add`].

[`overflowing_add`]: Self::overflowing_add

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
// Only the most significant word is signed.
//
//   10  MAX    (a = 10 × 2^24 + 2^24 - 1)
// + -5    9    (b = -5 × 2^24 + 9)
// ---------
//    6    8    (sum = 6 × 2^24 + 8)

let (a1, a0): (i32, u32) = (10_i32, u32::MAX);
let (b1, b0): (i32, u32) = (-5_i32, 9_u32);
let carry0 = false;

// uint::carrying_add for the less significant words
let (sum0, carry1) = a0.carrying_add(b0, carry0);
assert_eq!(carry1, true);

// int::carrying_add for the most significant word
let (sum1, overflow) = a1.carrying_add(b1, carry1);
assert_eq!(overflow, false);

assert_eq!((sum1, sum0), (6_i32, 8_u32));
# }
```
