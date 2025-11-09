Checks if the value is within the ASCII range.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let ascii = 97_u8;
let non_ascii = 150_u8;

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
# }
```
