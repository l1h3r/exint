Unchecked exact shift left. Computes `self << rhs`, assuming the operation
can be losslessly reversed and `rhs` cannot be larger than `Self::BITS`.

# Safety

This results in undefined behavior when `rhs > self.leading_zeros()` or
`rhs >= Self::BITS`, i.e. when [`shl_exact`] would return `None`.

[`shl_exact`]: Self::shl_exact
