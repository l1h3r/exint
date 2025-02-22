use ::core::debug_assert;
use ::core::intrinsics;
use ::core::mem;

/// Reinterprets the bits of a value of one type as another type.
///
/// # Safety
///
/// This results in undefined behaviour when `size_of::<T>() != size_of::<U>()`.
#[inline]
pub(crate) const unsafe fn transmute<T, U>(integer: T) -> U {
  debug_assert!(
    mem::size_of::<T>() == mem::size_of::<U>(),
    "attempt to transmute between types of different sizes",
  );

  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { intrinsics::transmute_unchecked(integer) }
}
