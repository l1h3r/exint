Checks if the value is an ASCII control character:
U+0000 NUL ..= U+001F UNIT SEPARATOR, or U+007F DELETE.
Note that most ASCII whitespace characters are control characters,
but SPACE is not.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
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

assert!(!uppercase_a.is_ascii_control());
assert!(!uppercase_g.is_ascii_control());
assert!(!a.is_ascii_control());
assert!(!g.is_ascii_control());
assert!(!zero.is_ascii_control());
assert!(!percent.is_ascii_control());
assert!(!space.is_ascii_control());
assert!(lf.is_ascii_control());
assert!(esc.is_ascii_control());
```
