Checks if the value is an ASCII decimal digit:
U+0030 '0' ..= U+0039 '9'.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let uppercase_a = u8::from(b'A');
let uppercase_g = u8::from(b'G');
let a = u8::from(b'a');
let g = u8::from(b'g');
let zero = u8::from(b'0');
let percent = u8::from(b'%');
let space = u8::from(b' ');
let lf = u8::from(b'\n');
let esc = u8::from(b'\x1b');

assert!(!uppercase_a.is_ascii_digit());
assert!(!uppercase_g.is_ascii_digit());
assert!(!a.is_ascii_digit());
assert!(!g.is_ascii_digit());
assert!(zero.is_ascii_digit());
assert!(!percent.is_ascii_digit());
assert!(!space.is_ascii_digit());
assert!(!lf.is_ascii_digit());
assert!(!esc.is_ascii_digit());
# }
```
