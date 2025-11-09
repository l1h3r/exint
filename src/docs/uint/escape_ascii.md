Returns an iterator that produces an escaped version of a `uint<1>`,
treating it as an ASCII character.

The behavior is identical to [`ascii::escape_default`].

[`ascii::escape_default`]: ::core::ascii::escape_default

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!("0", u8::from(b'0').escape_ascii().to_string());
assert_eq!("\\t", u8::from(b'\t').escape_ascii().to_string());
assert_eq!("\\r", u8::from(b'\r').escape_ascii().to_string());
assert_eq!("\\n", u8::from(b'\n').escape_ascii().to_string());
assert_eq!("\\'", u8::from(b'\'').escape_ascii().to_string());
assert_eq!("\\\"", u8::from(b'"').escape_ascii().to_string());
assert_eq!("\\\\", u8::from(b'\\').escape_ascii().to_string());
assert_eq!("\\x9d", u8::from(b'\x9d').escape_ascii().to_string());
# }
```
