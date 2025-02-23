Converts this value to its ASCII upper case equivalent in-place.

ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one,
use [`to_ascii_uppercase`].

[`to_ascii_uppercase`]: Self::to_ascii_uppercase

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let mut lowercase_a = uint!(b'a' u8);
let uppercase_a = uint!(b'A' u8);

lowercase_a.make_ascii_uppercase();

assert_eq!(uppercase_a, lowercase_a);
```
