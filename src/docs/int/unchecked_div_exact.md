Unchecked integer division without remainder. Computes `self / rhs`.

# Safety

This results in undefined behavior when `rhs == 0`, `self % rhs != 0`,
or `self == int::MIN && rhs == -1`, i.e. when [`checked_div_exact`] would return `None`.

[`checked_div_exact`]: Self::checked_div_exact
