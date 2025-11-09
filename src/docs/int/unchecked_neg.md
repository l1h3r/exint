Unchecked negation. Computes `-self`, assuming overflow cannot occur.

# Safety

This results in undefined behavior when `self == int::MIN`, i.e. when
[`checked_neg`] would return `None`.

[`checked_neg`]: Self::checked_neg
