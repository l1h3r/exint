Checks if the value is an ASCII graphic character:
U+0021 '!' ..= U+007E '~'.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let uppercase_a = uint!(b'A' u8);
let uppercase_g = uint!(b'G' u8);
let a = uint!(b'a' u8);
let g = uint!(b'g' u8);
let zero = uint!(b'0' u8);
let percent = uint!(b'%' u8);
let space = uint!(b' ' u8);
let lf = uint!(b'\n' u8);
let esc = uint!(b'\x1b' u8);

assert!(uppercase_a.is_ascii_graphic());
assert!(uppercase_g.is_ascii_graphic());
assert!(a.is_ascii_graphic());
assert!(g.is_ascii_graphic());
assert!(zero.is_ascii_graphic());
assert!(percent.is_ascii_graphic());
assert!(!space.is_ascii_graphic());
assert!(!lf.is_ascii_graphic());
assert!(!esc.is_ascii_graphic());
```
