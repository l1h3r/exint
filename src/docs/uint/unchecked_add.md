Unchecked integer addition. Computes `self + rhs`,
assuming overflow cannot occur.

Calling `x.unchecked_add(y)` is semantically equivalent to calling
`x.`[`checked_add`]`(y).`[`unwrap_unchecked`]`()`.

If you're just trying to avoid the panic in debug mode, then **do not** use
this. Instead, you're looking for [`wrapping_add`].

# Safety

This results in undefined behavior when `self + rhs > uint::MAX` or
`self + rhs < uint::MIN`, i.e. when [`checked_add`] would return `None`.

[`checked_add`]: Self::checked_add
[`wrapping_add`]: Self::wrapping_add
[`unwrap_unchecked`]: ::core::option::Option::unwrap_unchecked
