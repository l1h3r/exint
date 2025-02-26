Returns an iterator that produces an escaped version of a `uint<1>`,
treating it as an ASCII character.

The behavior is identical to [`ascii::escape_default`].

[`ascii::escape_default`]: ::core::ascii::escape_default

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!("0", uint!(b'0' u8).escape_ascii().to_string());
assert_eq!("\\t", uint!(b'\t' u8).escape_ascii().to_string());
assert_eq!("\\r", uint!(b'\r' u8).escape_ascii().to_string());
assert_eq!("\\n", uint!(b'\n' u8).escape_ascii().to_string());
assert_eq!("\\'", uint!(b'\'' u8).escape_ascii().to_string());
assert_eq!("\\\"", uint!(b'"' u8).escape_ascii().to_string());
assert_eq!("\\\\", uint!(b'\\' u8).escape_ascii().to_string());
assert_eq!("\\x9d", uint!(b'\x9d' u8).escape_ascii().to_string());
```
