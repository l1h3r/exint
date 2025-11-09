Converts an integer from big endian to the target's endianness.

On big endian this is a no-op. On little endian the bytes are swapped.

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
use exint::Saturating;

let n = Saturating(0x1A_i24);

if cfg!(target_endian = "big") {
    assert_eq!(Saturating::<i24>::from_be(n), n)
} else {
    assert_eq!(Saturating::<i24>::from_be(n), n.swap_bytes())
}
# }
```
