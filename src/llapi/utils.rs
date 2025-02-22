use crate::utils::Uint;

/// Sign digit for a two's complement integer.
///
/// - `0` indicates number is signed as positive.
/// - `1` indicates number is signed as negative.
pub(crate) const SIGN: u8 = 0b10000000;

/// Get the index of the least-significant byte from an [`Integer`] of size `N`.
///
/// [`Integer`]: crate::types::Integer
#[inline]
pub(crate) const fn lsb_index<const N: usize>() -> usize {
  #[cfg(target_endian = "big")]
  { N - 1 }

  #[cfg(target_endian = "little")]
  { 0 }
}

/// Get the index of the most-significant byte from an [`Integer`] of size `N`.
///
/// [`Integer`]: crate::types::Integer
#[inline]
pub(crate) const fn msb_index<const N: usize>() -> usize {
  #[cfg(target_endian = "big")]
  { 0 }

  #[cfg(target_endian = "little")]
  { N - 1 }
}

/// Resize an integer type to another of a different size.
#[inline]
pub(crate) const fn resize_bytes<T, const N: usize, const M: usize>(bytes: [u8; N]) -> [u8; M]
where
  T: Uint,
{
  let mut out: [u8; M] = [0; M];
  let mut src: *const u8 = bytes.as_ptr();
  let mut dst: *mut u8 = out.as_mut_ptr();

  if N < M {
    if !T::UINT {
      if bytes[msb_index::<N>()] & SIGN == SIGN {
        unsafe {
          ::core::ptr::write_bytes(dst, 0xFF, M);
        }
      }

      // ::core::panic!("TODO: sext")
    }

    // On BE machines, we need to shift the dst pointer to a higher memory
    // address leave the less-significant bytes untouched.
    if cfg!(target_endian = "big") {
      // SAFETY: We are still within the bounds of valid memory for `dst`
      //         because `dst == [u8; M]` and `(M - N) + N == M`.
      dst = unsafe { dst.add(M - N) };
    }

    // SAFETY: We know `N < M` since we ended up in this branch, this means
    //         `src` and `dst` are both valid for reads and writes of `N` bytes.
    //         We also know both pointers are plain byte arrays with identical
    //         alignment. Finally, since `src` was not yet deallocated, we can
    //         be sure the values do not overlap.
    unsafe { ::core::ptr::copy_nonoverlapping(src, dst, N) }
  } else {
    // On BE machines, we need to shift the src pointer to a higher memory
    // address and copy the less-significant bytes.
    if cfg!(target_endian = "big") {
      // SAFETY: We are still within the bounds of valid memory for `src`
      //         because `src == [u8; N]` and `(N - M) + M == N`.
      src = unsafe { src.add(N - M) };
    }

    // SAFETY: We know `N >= M` since we ended up in this branch, this means
    //         `src` and `dst` are both valid for reads and writes of `M` bytes.
    //         We also know both pointers are plain byte arrays with identical
    //         alignment. Finally, since `src` was not yet deallocated, we can
    //         be sure the values do not overlap.
    unsafe { ::core::ptr::copy_nonoverlapping(src, dst, M) }
  }

  out
}

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
/// [`transmute`]: ::core::mem::transmute
#[inline]
pub(crate) const unsafe fn transmute<T, U>(src: T) -> U {
  // TODO: Consider `core::assert_unsafe_precondition`
  ::core::debug_assert!(
    ::core::mem::size_of::<T>() == ::core::mem::size_of::<U>(),
    "cannot transmute between types of different sizes",
  );

  #[cfg(feature = "core_intrinsics")]
  #[inline]
  const fn __impl<T, U>(src: T) -> U {
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe { ::core::intrinsics::transmute_unchecked(src) }
  }

  // Borrowed from: https://github.com/rust-lang/project-safe-transmute
  #[cfg(not(feature = "core_intrinsics"))]
  #[inline]
  const fn __impl<T, U>(src: T) -> U {
    #[repr(C)]
    union Transmute<T, U> {
      src: ::core::mem::ManuallyDrop<T>,
      dst: ::core::mem::ManuallyDrop<U>,
    }

    // SAFETY: This is guaranteed to be safe by the caller.
    ::core::mem::ManuallyDrop::into_inner(unsafe {
      Transmute { src: ::core::mem::ManuallyDrop::new(src) }.dst
    })
  }

  __impl::<T, U>(src)
}
