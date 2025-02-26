Parses an integer from an ASCII-byte slice with decimal digits.

The characters are expected to be an optional `+` or `-` sign followed by only
digits. Leading and trailing non-digit characters (including whitespace)
represent an error. Underscores (which are accepted in Rust literals) also
represent an error.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert_eq!(int::from_ascii(b"+10"), Ok(int!(10)));
```

Trailing space returns error:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
assert!(int::from_ascii(b"1 ").is_err());
```
