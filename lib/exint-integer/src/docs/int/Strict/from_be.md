Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Strict;

let n = Strict(int!(0x1A));

if cfg!(target_endian = "big") {
    assert_eq!(<Strict<int>>::from_be(n), n)
} else {
    assert_eq!(<Strict<int>>::from_be(n), n.swap_bytes())
}
```
