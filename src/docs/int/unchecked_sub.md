Unchecked integer subtraction. Computes `self - rhs`,
assuming overflow cannot occur.

Calling `x.unchecked_sub(y)` is semantically equivalent to calling
`x.`[`checked_sub`]`(y).`[`unwrap_unchecked`]`()`.

If you're just trying to avoid the panic in debug mode, then **do not** use
this. Instead, you're looking for [`wrapping_sub`].

# Safety

This results in undefined behavior when `self - rhs > int::MAX` or
`self - rhs < int::MIN`, i.e. when [`checked_sub`] would return `None`.

[`checked_sub`]: Self::checked_sub
[`wrapping_sub`]: Self::wrapping_sub
[`unwrap_unchecked`]: ::core::option::Option::unwrap_unchecked
