Makes a copy of the value in its ASCII lower case equivalent.

ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`].

[`make_ascii_lowercase`]: Self::make_ascii_lowercase

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let uppercase_a = uint!(65 u8);
let lowercase_a = uint!(97 u8);

assert_eq!(lowercase_a, uppercase_a.to_ascii_lowercase());
```
