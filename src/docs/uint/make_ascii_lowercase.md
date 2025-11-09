Converts this value to its ASCII lower case equivalent in-place.

ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one,
use [`to_ascii_lowercase`].

[`to_ascii_lowercase`]: Self::to_ascii_lowercase

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let mut uppercase_a = u8::from(b'A');
let lowercase_a = u8::from(b'a');

uppercase_a.make_ascii_lowercase();

assert_eq!(lowercase_a, uppercase_a);
# }
```
