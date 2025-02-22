Converts an integer from little endian to the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
# #![allow(non_camel_case_types)]
# type uint = exint::uint<4>;
# type int  = exint::int<4>;
# use exint::*;
use exint::Saturating;

let n = Saturating(uint!(0x1A));

if cfg!(target_endian = "little") {
    assert_eq!(<Saturating<uint>>::from_le(n), n)
} else {
    assert_eq!(<Saturating<uint>>::from_le(n), n.swap_bytes())
}
```
