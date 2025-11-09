Unchecked integer multiplication. Computes `self * rhs`,
assuming overflow cannot occur.

Calling `x.unchecked_mul(y)` is semantically equivalent to calling
`x.`[`checked_mul`]`(y).`[`unwrap_unchecked`]`()`.

If you're just trying to avoid the panic in debug mode, then **do not** use
this. Instead, you're looking for [`wrapping_mul`].

# Safety

This results in undefined behavior when `self * rhs > int::MAX` or
`self * rhs < int::MIN`, i.e. when [`checked_mul`] would return `None`.

[`checked_mul`]: Self::checked_mul
[`wrapping_mul`]: Self::wrapping_mul
[`unwrap_unchecked`]: ::core::option::Option::unwrap_unchecked
