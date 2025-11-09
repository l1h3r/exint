Converts `self` to big endian from the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

let n = Wrapping(0x1A_u24);

if cfg!(target_endian = "big") {
    assert_eq!(n.to_be(), n)
} else {
    assert_eq!(n.to_be(), n.swap_bytes())
}
# }
```
