Makes a copy of the value in its ASCII upper case equivalent.

ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`].

[`make_ascii_uppercase`]: Self::make_ascii_uppercase

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let lowercase_a = uint!(97 u8);
let uppercase_a = uint!(65 u8);

assert_eq!(uppercase_a, lowercase_a.to_ascii_uppercase());
```
