Converts an integer from little endian to the target's endianness.

On little endian this is a no-op. On big endian the bytes are swapped.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let n = 0x1A_i24;

if cfg!(target_endian = "little") {
    assert_eq!(i24::from_le(n), n)
} else {
    assert_eq!(i24::from_le(n), n.swap_bytes())
}
# }
```
