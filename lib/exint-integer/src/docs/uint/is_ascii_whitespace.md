Checks if the value is an ASCII whitespace character:
U+0020 SPACE, U+0009 HORIZONTAL TAB, U+000A LINE FEED,
U+000C FORM FEED, or U+000D CARRIAGE RETURN.

See [`u8::is_ascii_whitespace`] for more information.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let uppercase_a = uint!(b'A' u8);
let uppercase_g = uint!(b'G' u8);
let a = uint!(b'a' u8);
let g = uint!(b'g' u8);
let zero = uint!(b'0' u8);
let percent = uint!(b'%' u8);
let space = uint!(b' ' u8);
let lf = uint!(b'\n' u8);
let esc = uint!(b'\x1b' u8);

assert!(!uppercase_a.is_ascii_whitespace());
assert!(!uppercase_g.is_ascii_whitespace());
assert!(!a.is_ascii_whitespace());
assert!(!g.is_ascii_whitespace());
assert!(!zero.is_ascii_whitespace());
assert!(!percent.is_ascii_whitespace());
assert!(space.is_ascii_whitespace());
assert!(lf.is_ascii_whitespace());
assert!(!esc.is_ascii_whitespace());
```
