//! Common utility functions, for internal use only.

use crate::traits::Consts;
use crate::types::Integer;

/// Reinterprets the bits of a value of one type as another type.
///
/// This is like [`transmute`], but extra dangerous since it makes no
/// compile-time guarantees that `size_of::<T>() == size_of::<U>()`
///
/// Prefer normal `transmute` whenever possible.
///
/// # Panics
///
/// This function panics in non optimized builds when `T` and `U` have different
/// sizes.
///
/// # Safety
///
/// This results in undefined behaviour when `size_of::<T>() != size_of::<U>()`.
///
/// [`transmute`]: ::core::intrinsics::transmute
#[inline]
pub(crate) const unsafe fn transmute<T, U>(src: T) -> U {
  ::core::debug_assert!(
    ::core::mem::size_of::<T>() == ::core::mem::size_of::<U>(),
    "cannot transmute between types of different sizes",
  );

  // SAFETY: This is guaranteed to be safe by the caller.
  unsafe { ::core::intrinsics::transmute_unchecked(src) }
}

/// Get the index of the most-significant byte from an [`Integer`] of size `S`.
///
/// [`Integer`]: crate::types::Integer
#[inline]
pub(crate) const fn msb_index<const S: usize>() -> usize {
  #[cfg(target_endian = "big")]
  { 0 }

  #[cfg(target_endian = "little")]
  { S - 1 }
}

/// Get the index of the least-significant byte from an [`Integer`] of size `S`.
///
/// [`Integer`]: crate::types::Integer
#[inline]
pub(crate) const fn lsb_index<const S: usize>() -> usize {
  #[cfg(target_endian = "big")]
  { S - 1 }

  #[cfg(target_endian = "little")]
  { 0 }
}

/// Resize an integer type to another of a different size.
#[inline]
pub(crate) const fn resize<const T: usize, const U: usize, const UINT: bool>(
  integer: Integer<T>,
) -> Integer<U> {
  let mut out: Integer<U> = Integer::UMIN;
  let mut src: *const u8 = integer.as_ptr();
  let mut dst: *mut u8 = out.as_mut_ptr();

  if T < U {
    if !UINT {
      ::core::panic!("TODO: Handle sext")
    }

    // In BE, we need to shift the dst pointer to a higher memory address and
    // leave the less-significant bytes zeroed.
    //
    // SAFETY: Reads are safe since `dst == [u8; U]` and `(U - T) + T == U`
    if cfg!(target_endian = "big") {
      dst = unsafe { dst.add(U - T) };
    }

    // SAFETY:
    //   - Both `src` and `dst` are valid for reads and write of *at least* `T`
    //   - Alignment is identical because `src` and `dst` are both byte arrays
    //   - The values *do not* overlap - `dst` was allocated within this function
    unsafe {
      ::core::ptr::copy_nonoverlapping(src, dst, T);
    }
  } else {
    // In BE, we need to shift the src pointer to a higher memory address and
    // copy the less-significant bytes.
    //
    // SAFETY: Reads are safe since `src == [u8; T]` and `(T - U) + U == T`
    if cfg!(target_endian = "big") {
      src = unsafe { src.add(T - U) };
    }

    // SAFETY:
    //   - Both `src` and `dst` are valid for reads and write of *at least* `U`
    //   - Alignment is identical because `src` and `dst` are both byte arrays
    //   - The values *do not* overlap - `dst` was allocated within this function
    unsafe {
      ::core::ptr::copy_nonoverlapping(src, dst, U);
    }
  }

  out
}
