Unchecked exact shift right. Computes `self >> rhs`, assuming the operation
can be losslessly reversed and `rhs` cannot be larger than `Self::BITS`.

# Safety

This results in undefined behavior when `rhs > self.trailing_zeros()` or
`rhs >= Self::BITS`, i.e. when [`shr_exact`] would return `None`.

[`shr_exact`]: Self::shr_exact
