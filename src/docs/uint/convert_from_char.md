Converts a [`char`] into a [`uint`].

[`uint`]: crate::types::uint

# Examples

Basic usage:

```
# use ::exint::primitive::*;
# ::exint::uint! {
assert!(4 == size_of_val(&u32::from('c')));
assert!(4 == size_of_val(&u32::from('👤')));
assert!(4 == size_of_val(&u32::from('⚙')));
# }
```
