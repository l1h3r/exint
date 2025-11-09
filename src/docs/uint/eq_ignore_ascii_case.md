Checks that two values are an ASCII case-insensitive match.

This is equivalent to `to_ascii_lowercase(a) == to_ascii_lowercase(b)`.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let lowercase_a = u8::from(b'a');
let uppercase_a = u8::from(b'A');

assert!(lowercase_a.eq_ignore_ascii_case(&uppercase_a));
# }
```
