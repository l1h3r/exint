Checks if the value is an ASCII octal digit:
U+0030 '0' ..= U+0037 '7'.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let uppercase_a = uint!(b'A' u8);
let a = uint!(b'a' u8);
let zero = uint!(b'0' u8);
let seven = uint!(b'7' u8);
let nine = uint!(b'9' u8);
let percent = uint!(b'%' u8);
let lf = uint!(b'\n' u8);

assert!(!uppercase_a.is_ascii_octdigit());
assert!(!a.is_ascii_octdigit());
assert!(zero.is_ascii_octdigit());
assert!(seven.is_ascii_octdigit());
assert!(!nine.is_ascii_octdigit());
assert!(!percent.is_ascii_octdigit());
assert!(!lf.is_ascii_octdigit());
```
