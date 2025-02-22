use crate::types::int;
use crate::types::uint;

/// A marker trait for integer types.
///
/// # Safety
///
/// Types implementing this trait must ensure they are valid representations of
/// integers with no padding or uninitialized bytes.
pub(crate) unsafe trait Uint: ::core::marker::Copy {
  const UINT: bool;
}

unsafe impl<const N: usize> Uint for int<N> {
  const UINT: bool = false;
}

unsafe impl<const N: usize> Uint for uint<N> {
  const UINT: bool = true;
}

macro_rules! implement {
  ($type:ty, $uint:literal) => {
    unsafe impl Uint for $type {
      const UINT: bool = $uint;
    }
  };
  ($($type:ty)+, $uint:literal) => {
    $(
      implement!($type, $uint);
    )+
  };
}

implement!(i8 i16 i32 i64 i128 isize, false);
implement!(u8 u16 u32 u64 u128 usize, true);
