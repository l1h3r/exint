Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Wrapping;

let n = Wrapping(int!(0x1A));

if cfg!(target_endian = "big") {
    assert_eq!(<Wrapping<int>>::from_be(n), n)
} else {
    assert_eq!(<Wrapping<int>>::from_be(n), n.swap_bytes())
}
```
