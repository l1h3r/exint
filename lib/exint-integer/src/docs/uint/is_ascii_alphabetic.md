Checks if the value is an ASCII alphabetic character:

- U+0041 'A' ..= U+005A 'Z', or
- U+0061 'a' ..= U+007A 'z'.

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

assert!(uppercase_a.is_ascii_alphabetic());
assert!(uppercase_g.is_ascii_alphabetic());
assert!(a.is_ascii_alphabetic());
assert!(g.is_ascii_alphabetic());
assert!(!zero.is_ascii_alphabetic());
assert!(!percent.is_ascii_alphabetic());
assert!(!space.is_ascii_alphabetic());
assert!(!lf.is_ascii_alphabetic());
assert!(!esc.is_ascii_alphabetic());
```
