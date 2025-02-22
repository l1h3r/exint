Parses an integer from an ASCII-byte slice with digits in a given base.

The characters are expected to be an optional `+` sign followed by only digits.
Leading and trailing non-digit characters (including whitespace) represent an
error. Underscores (which are accepted in Rust literals) also represent an error.

Digits are a subset of these characters, depending on `radix`:
* `0-9`
* `a-z`
* `A-Z`

# Panics

This function panics if `radix` is not in the range from 2 to 36.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert_eq!(uint::from_ascii_radix(b"A", 16), Ok(uint!(10)));
```

Trailing space returns error:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
assert!(uint::from_ascii_radix(b"1 ", 10).is_err());
```
