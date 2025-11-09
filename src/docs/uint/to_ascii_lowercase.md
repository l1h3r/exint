Makes a copy of the value in its ASCII lower case equivalent.

ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`].

[`make_ascii_lowercase`]: Self::make_ascii_lowercase

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let uppercase_a = 65_u8;
let lowercase_a = 97_u8;

assert_eq!(lowercase_a, uppercase_a.to_ascii_lowercase());
# }
```
