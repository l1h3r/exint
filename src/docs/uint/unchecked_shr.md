Unchecked shift right. Computes `self >> rhs`,
assuming that `rhs` is less than the number of bits in `self`.

# Safety

This results in undefined behavior when `rhs` is larger than or equal to the
number of bits in `self`, i.e. when [`checked_shr`] would return `None`.

[`checked_shr`]: Self::checked_shr
