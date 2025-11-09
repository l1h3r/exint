use ::core::debug_assert;
use ::core::marker::Copy;
use ::core::mem::size_of;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Sign digit for a two's complement integer.
///
/// - `0` indicates number is signed as positive.
/// - `1` indicates number is signed as negative.
pub(crate) const SIGN: u8 = 0b1000_0000;

// -----------------------------------------------------------------------------
// Uint
// -----------------------------------------------------------------------------

/// A marker trait for integer types.
///
/// # Safety
///
/// Types implementing this trait must ensure they are valid representations of
/// integers with no padding or uninitialized bytes.
pub unsafe trait Uint: Copy {}

// SAFETY: byte arrays don't have padding or uninitialized bytes.
unsafe impl<const N: usize> Uint for [u8; N] {}

macro_rules! implement_uint {
  ($($type:ty)+) => {
    $(
      // SAFETY: primitive integer type.
      unsafe impl Uint for $type {}
    )+
  };
}

implement_uint! {
  i8 i16 i32 i64 i128 isize
  u8 u16 u32 u64 u128 usize
}

// -----------------------------------------------------------------------------
// Dynamic Constants
// -----------------------------------------------------------------------------

/// Supporting trait for constant integer values.
pub(crate) trait Consts {
  /// The size of this integer type in bytes.
  const SIZE: usize;
  /// The size of this integer type in bits.
  const BITS: u32;
  /// The largest unsigned value that can be represented by this integer type.
  const UMAX: Self;
  /// The smallest unsigned value that can be represented by this integer type.
  const UMIN: Self;
  /// The largest signed value that can be represented by this integer type.
  const SMAX: Self;
  /// The smallest signed value that can be represented by this integer type.
  const SMIN: Self;
  /// The value `1` represented by this integer type.
  const ONE: Self;
}

impl<T: Uint> Consts for T {
  const SIZE: usize = size_of::<T>();
  const BITS: u32 = (Self::SIZE as u32) << 3;

  const UMAX: Self = maybe_uninit_fill(u8::MAX);
  const UMIN: Self = maybe_uninit_fill(u8::MIN);

  const SMAX: Self = {
    let mut value: Self = Self::UMAX;
    let slice: &mut [u8] = maybe_uninit_slice(&mut value);
    let index: usize = Index::ZERO.msb_of(Self::SIZE);

    slice[index] ^= SIGN;
    value
  };

  const SMIN: Self = {
    let mut value: Self = Self::UMIN;
    let slice: &mut [u8] = maybe_uninit_slice(&mut value);
    let index: usize = Index::ZERO.msb_of(Self::SIZE);

    slice[index] |= SIGN;
    value
  };

  const ONE: Self = {
    let mut value: Self = Self::UMIN;
    let slice: &mut [u8] = maybe_uninit_slice(&mut value);
    let index: usize = Index::ZERO.lsb_of(Self::SIZE);

    slice[index] = 1;
    value
  };
}

#[inline]
const fn maybe_uninit_fill<T: Uint>(fill: u8) -> T {
  let mut this: ::core::mem::MaybeUninit<T> = ::core::mem::MaybeUninit::uninit();

  unsafe {
    this.as_mut_ptr().write_bytes(fill, 1);
  }

  unsafe { this.assume_init() }
}

const fn maybe_uninit_slice<T: Uint>(value: &mut T) -> &mut [u8] {
  let ptr: *mut T = ::core::ptr::from_mut(value);
  let len: usize = ::core::mem::size_of::<T>();

  unsafe { ::core::slice::from_raw_parts_mut(ptr.cast(), len) }
}

// -----------------------------------------------------------------------------
// Index
// -----------------------------------------------------------------------------

#[repr(transparent)]
pub(crate) struct Index(pub(crate) usize);

impl Index {
  pub(crate) const ZERO: Self = Self(0);

  #[cfg(target_endian = "big")]
  #[inline]
  pub(crate) const fn lsb_of(self, size: usize) -> usize {
    size - 1 - self.0
  }
  #[cfg(target_endian = "little")]
  #[inline]
  pub(crate) const fn lsb_of(self, _size: usize) -> usize {
    self.0
  }

  #[cfg(target_endian = "big")]
  #[inline]
  pub(crate) const fn msb_of(self, _size: usize) -> usize {
    self.0
  }
  #[cfg(target_endian = "little")]
  #[inline]
  pub(crate) const fn msb_of(self, size: usize) -> usize {
    size - 1 - self.0
  }
}

// -----------------------------------------------------------------------------
// Transmute
// -----------------------------------------------------------------------------

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

  debug_assert!(
    size_of::<T>() == size_of::<U>(),
    "cannot transmute between types of different sizes",
  );

  __impl::<T, U>(src)
}
