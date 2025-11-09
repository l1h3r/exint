Checks if the value is an ASCII punctuation character:

- U+0021 ..= U+002F `! " # $ % & ' ( ) * + , - . /`, or
- U+003A ..= U+0040 `: ; < = > ? @`, or
- U+005B ..= U+0060 `` [ \ ] ^ _ ` ``, or
- U+007B ..= U+007E `{ | } ~`

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

assert!(!uppercase_a.is_ascii_punctuation());
assert!(!uppercase_g.is_ascii_punctuation());
assert!(!a.is_ascii_punctuation());
assert!(!g.is_ascii_punctuation());
assert!(!zero.is_ascii_punctuation());
assert!(percent.is_ascii_punctuation());
assert!(!space.is_ascii_punctuation());
assert!(!lf.is_ascii_punctuation());
assert!(!esc.is_ascii_punctuation());
# }
```
