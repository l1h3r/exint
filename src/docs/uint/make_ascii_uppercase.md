Converts this value to its ASCII upper case equivalent in-place.

ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one,
use [`to_ascii_uppercase`].

[`to_ascii_uppercase`]: Self::to_ascii_uppercase

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let mut lowercase_a = u8::from(b'a');
let uppercase_a = u8::from(b'A');

lowercase_a.make_ascii_uppercase();

assert_eq!(uppercase_a, lowercase_a);
# }
```
