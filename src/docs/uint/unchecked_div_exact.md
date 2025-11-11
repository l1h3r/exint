Unchecked integer division without remainder. Computes `self / rhs`.

# Safety

This results in undefined behavior when `rhs == 0` or `self % rhs != 0`,
i.e. when [`checked_div_exact`] would return `None`.

[`checked_div_exact`]: Self::checked_div_exact
