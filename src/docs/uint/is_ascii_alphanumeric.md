Checks if the value is an ASCII alphanumeric character:

- U+0041 'A' ..= U+005A 'Z', or
- U+0061 'a' ..= U+007A 'z', or
- U+0030 '0' ..= U+0039 '9'.

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

assert!(uppercase_a.is_ascii_alphanumeric());
assert!(uppercase_g.is_ascii_alphanumeric());
assert!(a.is_ascii_alphanumeric());
assert!(g.is_ascii_alphanumeric());
assert!(zero.is_ascii_alphanumeric());
assert!(!percent.is_ascii_alphanumeric());
assert!(!space.is_ascii_alphanumeric());
assert!(!lf.is_ascii_alphanumeric());
assert!(!esc.is_ascii_alphanumeric());
# }
```
