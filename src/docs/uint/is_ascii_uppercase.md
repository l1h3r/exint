Checks if the value is an ASCII uppercase character:
U+0041 'A' ..= U+005A 'Z'.

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

assert!(uppercase_a.is_ascii_uppercase());
assert!(uppercase_g.is_ascii_uppercase());
assert!(!a.is_ascii_uppercase());
assert!(!g.is_ascii_uppercase());
assert!(!zero.is_ascii_uppercase());
assert!(!percent.is_ascii_uppercase());
assert!(!space.is_ascii_uppercase());
assert!(!lf.is_ascii_uppercase());
assert!(!esc.is_ascii_uppercase());
# }
```
