Unchecked exact shift left. Computes `self << rhs`, assuming the operation
can be losslessly reversed and `rhs` cannot be larger than `Self::BITS`.

# Safety

This results in undefined behavior when `rhs >= self.leading_zeros()` and
`rhs >= self.leading_ones()`, i.e. when [`exact_shl`] would return `None`.

[`exact_shl`]: Self::exact_shl
