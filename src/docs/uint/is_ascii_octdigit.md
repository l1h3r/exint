Checks if the value is an ASCII octal digit:
U+0030 '0' ..= U+0037 '7'.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let uppercase_a = u8::from(b'A');
let a = u8::from(b'a');
let zero = u8::from(b'0');
let seven = u8::from(b'7');
let nine = u8::from(b'9');
let percent = u8::from(b'%');
let lf = u8::from(b'\n');

assert!(!uppercase_a.is_ascii_octdigit());
assert!(!a.is_ascii_octdigit());
assert!(zero.is_ascii_octdigit());
assert!(seven.is_ascii_octdigit());
assert!(!nine.is_ascii_octdigit());
assert!(!percent.is_ascii_octdigit());
assert!(!lf.is_ascii_octdigit());
# }
```
