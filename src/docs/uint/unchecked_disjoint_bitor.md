Same value as `self | rhs`, but UB if any bit position is set in both inputs.

This is a situational micro-optimization for places where you'd rather use
addition on some platforms and bitwise or on other platforms, based on exactly
which instructions combine better with whatever else you're doing. Note that
there's no reason to bother using this for places where it's clear from the
operations involved that they can't overlap. For example, if you're combining
`u16`s into a `u32` with `((a as u32) << 16) | (b as u32)`, that's fine, as the
backend will know those sides of the `|` are disjoint without needing help.

# Safety

This results in undefined behavior when `(self & rhs) != 0`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
// SAFETY: `1` and `4` have no bits in common.
unsafe {
    assert_eq!(1_u24.unchecked_disjoint_bitor(4_u24), 5_u24);
}
# }
```
