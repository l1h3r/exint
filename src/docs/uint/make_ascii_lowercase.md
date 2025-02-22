Converts this value to its ASCII lower case equivalent in-place.

ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one,
use [`to_ascii_lowercase`].

[`to_ascii_lowercase`]: Self::to_ascii_lowercase

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
let mut uppercase_a = uint!(b'A' u8);
let lowercase_a = uint!(b'a' u8);

uppercase_a.make_ascii_lowercase();

assert_eq!(lowercase_a, uppercase_a);
```
