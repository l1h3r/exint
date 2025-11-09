Parses an integer from an ASCII-byte slice with decimal digits.

The characters are expected to be an optional `+` sign followed by only digits.
Leading and trailing non-digit characters (including whitespace) represent an
error. Underscores (which are accepted in Rust literals) also represent an error.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert_eq!(u24::from_ascii(b"+10"), Ok(10_u24));
# }
```

Trailing space returns error:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(u24::from_ascii(b"1 ").is_err());
# }
```
