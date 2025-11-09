Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
let n = 0x1A_u24;

if cfg!(target_endian = "big") {
    assert_eq!(u24::from_be(n), n)
} else {
    assert_eq!(u24::from_be(n), n.swap_bytes())
}
# }
```
