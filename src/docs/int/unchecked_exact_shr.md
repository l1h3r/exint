Unchecked exact shift right. Computes `self >> rhs`, assuming the operation
can be losslessly reversed and `rhs` cannot be larger than `Self::BITS`.

# Safety

This results in undefined behavior when `rhs > self.trailing_zeros()` or
`rhs >= Self::BITS`, i.e. when [`exact_shr`] would return `None`.

[`exact_shr`]: Self::exact_shr
