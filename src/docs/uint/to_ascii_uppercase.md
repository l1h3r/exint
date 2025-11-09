Makes a copy of the value in its ASCII upper case equivalent.

ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`].

[`make_ascii_uppercase`]: Self::make_ascii_uppercase

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let lowercase_a = 97_u8;
let uppercase_a = 65_u8;

assert_eq!(uppercase_a, lowercase_a.to_ascii_uppercase());
# }
```
