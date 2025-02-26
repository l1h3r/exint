Checks if the value is within the ASCII range.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
let ascii = uint!(97 u8);
let non_ascii = uint!(150 u8);

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```
