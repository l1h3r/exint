Unchecked shift left. Computes `self << rhs`,
assuming that `rhs` is less than the number of bits in `self`.

# Safety

This results in undefined behavior if `rhs` is larger than or equal to the
number of bits in `self`, i.e. when [`checked_shl`] would return `None`.

[`checked_shl`]: Self::checked_shl
