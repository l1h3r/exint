Checks if the value is an ASCII whitespace character:
U+0020 SPACE, U+0009 HORIZONTAL TAB, U+000A LINE FEED,
U+000C FORM FEED, or U+000D CARRIAGE RETURN.

See [`u8::is_ascii_whitespace`] for more information.

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

assert!(!uppercase_a.is_ascii_whitespace());
assert!(!uppercase_g.is_ascii_whitespace());
assert!(!a.is_ascii_whitespace());
assert!(!g.is_ascii_whitespace());
assert!(!zero.is_ascii_whitespace());
assert!(!percent.is_ascii_whitespace());
assert!(space.is_ascii_whitespace());
assert!(lf.is_ascii_whitespace());
assert!(!esc.is_ascii_whitespace());
# }
```
