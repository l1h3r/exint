Converts `self` to little endian from the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Wrapping;

let n = Wrapping(0x1A_u24);

if cfg!(target_endian = "little") {
    assert_eq!(n.to_le(), n)
} else {
    assert_eq!(n.to_le(), n.swap_bytes())
}
# }
```
