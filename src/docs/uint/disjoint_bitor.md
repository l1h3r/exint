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
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
// SAFETY: `1` and `4` have no bits in common.
unsafe {
    assert_eq!(uint!(1).unchecked_disjoint_bitor(uint!(4)), uint!(5));
}
```
