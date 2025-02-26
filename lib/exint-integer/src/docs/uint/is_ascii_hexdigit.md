Checks if the value is an ASCII hexadecimal digit:

- U+0030 '0' ..= U+0039 '9', or
- U+0041 'A' ..= U+0046 'F', or
- U+0061 'a' ..= U+0066 'f'.

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

assert!(uppercase_a.is_ascii_hexdigit());
assert!(!uppercase_g.is_ascii_hexdigit());
assert!(a.is_ascii_hexdigit());
assert!(!g.is_ascii_hexdigit());
assert!(zero.is_ascii_hexdigit());
assert!(!percent.is_ascii_hexdigit());
assert!(!space.is_ascii_hexdigit());
assert!(!lf.is_ascii_hexdigit());
assert!(!esc.is_ascii_hexdigit());
```
