Converts an integer from little endian to the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# #[macro_use] extern crate exint_integer as exint;
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
use exint::Wrapping;

let n = Wrapping(int!(0x1A));

if cfg!(target_endian = "little") {
    assert_eq!(<Wrapping<int>>::from_le(n), n)
} else {
    assert_eq!(<Wrapping<int>>::from_le(n), n.swap_bytes())
}
```
