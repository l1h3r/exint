Parses an integer from a string slice with digits in a given base.

The string is expected to be an optional `+` or `-` sign followed by only digits.
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
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(i24::from_str_radix("A", 16), Ok(10_i24));
# }
```

Trailing space returns error:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(i24::from_str_radix("1 ", 10).is_err());
# }
```
