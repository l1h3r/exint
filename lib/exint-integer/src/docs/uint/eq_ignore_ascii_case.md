Checks that two values are an ASCII case-insensitive match.

This is equivalent to `to_ascii_lowercase(a) == to_ascii_lowercase(b)`.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let lowercase_a = uint!(b'a' u8);
let uppercase_a = uint!(b'A' u8);

assert!(lowercase_a.eq_ignore_ascii_case(&uppercase_a));
```
