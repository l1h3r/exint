//! Specialization for (`0..=16`)-bit Non-Standard Integers
//!
//! Implementations for generic integers that have a **larger** built-in primitive type.
//!
//! The following implementations are defined:
//!   - [`I24`]
//!     - required size: `3`
//!   - [`I40`]
//!     - required size: `5`
//!   - [`I48`]
//!     - required size: `6`
//!   - [`I56`]
//!     - required size: `7`
//!   - [`I72`]
//!     - required size: `9`
//!   - [`I80`]
//!     - required size: `10`
//!   - [`I88`]
//!     - required size: `11`
//!   - [`I96`]
//!     - required size: `12`
//!   - [`I104`]
//!     - required size: `13`
//!   - [`I112`]
//!     - required size: `14`
//!   - [`I120`]
//!     - required size: `15`
//!
//! # Safety
//!
//! The functions here expect the externally-provided generic integers to have
//! the same size as described above.
//!
//! All invariants of [`Uint`] are required.
//!
//! # Additional
//!
//! The implementations are tuned to generate ideal LLVM IR with appropriately
//! narrowed instructions (ie. `mul i24`, `@llvm.uadd.sat.i40`).
//!
//! This is generally done by using patterns recognized by LLVM or by extending
//! to a larger built-in primitive type and transforming the output accordingly.
//!
//! [`Uint`]: crate::llapi::Uint

macro_rules! define_extended_integer {
  ($name:ident => $transform:ident, $size:literal, $uint:ty, $sint:ty) => {
    #[repr(C, packed)]
    struct $transform {
      #[cfg(target_endian = "big")]
      padding: [u8; Self::PADDING],
      integer: [u8; $size],
      #[cfg(target_endian = "little")]
      padding: [u8; Self::PADDING],
    }

    impl $transform {
      const BITDIFF: u32 = <$uint>::BITS - $name::BITS;
      const PADDING: usize = ::core::mem::size_of::<$uint>() - $name::SIZE;

      #[inline]
      const fn from_array(integer: [u8; $size]) -> Self {
        Self {
          integer,
          padding: [0; Self::PADDING],
        }
      }

      #[inline]
      const fn from_uint(value: $uint) -> Self {
        unsafe { ::core::mem::transmute(value) }
      }

      #[inline]
      const fn from_sint(value: $sint) -> Self {
        unsafe { ::core::mem::transmute(value) }
      }
    }

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub(crate) struct $name([u8; $size]);

    // SAFETY: `$name` has the same layout and ABI as a primitive integer type.
    unsafe impl $crate::llapi::utils::Uint for $name {}

    impl $name {
      const SIZE: usize = <$name as $crate::llapi::utils::Consts>::SIZE;
      const BITS: u32 = <$name as $crate::llapi::utils::Consts>::BITS;

      const UMAX: $uint = Self::zext(<$name as $crate::llapi::utils::Consts>::UMAX);

      const SMAX: $sint = Self::sext(<$name as $crate::llapi::utils::Consts>::SMAX);
      const SMIN: $sint = Self::sext(<$name as $crate::llapi::utils::Consts>::SMIN);

      #[inline]
      const fn zext(self) -> $uint {
        unsafe { ::core::mem::transmute($transform::from_array(self.0)) }
      }

      #[inline]
      const fn sext(self) -> $sint {
        (self.zext().cast_signed() << $transform::BITDIFF) >> $transform::BITDIFF
      }

      #[inline]
      const fn read_uint(value: $uint) -> Self {
        Self($transform::from_uint(value).integer)
      }

      #[inline]
      const fn read_sint(value: $sint) -> Self {
        Self($transform::from_sint(value).integer)
      }

      // -----------------------------------------------------------------------
      // Bitwise Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn band(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(lhs.zext() & rhs.zext())
      }

      #[inline]
      pub(crate) const fn bor(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(lhs.zext() | rhs.zext())
      }

      #[inline]
      pub(crate) const fn bxor(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(lhs.zext() ^ rhs.zext())
      }

      #[inline]
      pub(crate) const fn bnot(int: Self) -> Self {
        Self::read_uint(int.zext() ^ Self::UMAX)
      }

      // -----------------------------------------------------------------------
      // Comparison Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn eq(lhs: Self, rhs: Self) -> bool {
        lhs.zext() == rhs.zext()
      }

      #[inline]
      pub(crate) const fn ucmp(lhs: Self, rhs: Self) -> ::core::cmp::Ordering {
        $crate::llapi::intrinsics::three_way_compare!(lhs.zext(), rhs.zext())
      }

      #[inline]
      pub(crate) const fn scmp(lhs: Self, rhs: Self) -> ::core::cmp::Ordering {
        $crate::llapi::intrinsics::three_way_compare!(lhs.sext(), rhs.sext())
      }

      // -----------------------------------------------------------------------
      // Bit Conversion Operations
      // -----------------------------------------------------------------------

      // Note: LLVM only recognizes this pattern when increasing the loop unroll
      //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
      #[inline]
      pub(crate) const fn swap1(int: Self) -> Self {
        let mut input: $uint = int.zext();
        let mut value: $uint = 0;
        let mut index: usize = 0;

        while index < Self::BITS as usize {
          value = (value << 1) | (input & 1);
          input >>= 1;
          index += 1;
        }

        Self::read_uint(value)
      }

      #[inline]
      pub(crate) const fn swap8(int: Self) -> Self {
        Self::read_uint($crate::llapi::intrinsics::bswap!(int.zext() << $transform::BITDIFF))
      }

      #[inline]
      pub(crate) const fn rotl(int: Self, bits: u32) -> Self {
        // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
        unsafe { Self::unchecked_fshl(int, int, bits % Self::BITS) }
      }

      #[inline]
      pub(crate) const fn rotr(int: Self, bits: u32) -> Self {
        // SAFETY: We mask the shift value so we cannot shift out-of-bounds.
        unsafe { Self::unchecked_fshr(int, int, bits % Self::BITS) }
      }

      // -----------------------------------------------------------------------
      // Bit Inspection Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn ctpop(int: Self) -> u32 {
        $crate::llapi::intrinsics::ctpop!(int.zext())
      }

      // Note: LLVM only recognizes this pattern when increasing the loop unroll
      //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
      #[inline]
      pub(crate) const fn ctlz(int: Self) -> u32 {
        Self::cttz(Self::swap1(int))
      }

      #[inline]
      pub(crate) const fn cttz(int: Self) -> u32 {
        if int.zext() == 0 {
          Self::BITS
        } else {
          $crate::llapi::intrinsics::cttz!(int.zext())
        }
      }

      // Note: LLVM only recognizes this pattern when increasing the loop unroll
      //       threshold with the following: `-C llvm-args=-unroll-threshold=n`
      #[inline]
      pub(crate) const unsafe fn ctlz_nonzero(int: Self) -> u32 {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { Self::cttz_nonzero(Self::swap1(int)) }
      }

      #[inline]
      pub(crate) const unsafe fn cttz_nonzero(int: Self) -> u32 {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe { $crate::llapi::intrinsics::cttz_nonzero!(int.zext()) }
      }

      // -----------------------------------------------------------------------
      // Overflowing Operations
      // -----------------------------------------------------------------------

      // LLVM generates `add $type` and `icmp ult $type` instructions
      //
      // Note: This appears to be the recommended pattern:
      //   https://github.com/rust-lang/rust/pull/124114#issuecomment-2066173305
      #[inline]
      pub(crate) const fn overflowing_uadd(lhs: Self, rhs: Self) -> (Self, bool) {
        let out: Self = Self::wrapping_add(lhs, rhs);
        let cmp: bool = Self::ucmp(out, lhs).is_lt();

        (out, cmp)
      }

      // Note: LLVM generates `sub $type` and `icmp ult $type` which is
      //       considered the canonical form of `usub.with.overflow`.
      //       More info here: https://github.com/rust-lang/rust/pull/103299
      #[inline]
      pub(crate) const fn overflowing_usub(lhs: Self, rhs: Self) -> (Self, bool) {
        // Note: This order (cmp -> sub) seems essential for i40/i48/i56 optimizations
        let cmp: bool = Self::ucmp(lhs, rhs).is_lt();
        let out: Self = Self::wrapping_sub(lhs, rhs);

        (out, cmp)
      }

      // Note: LLVM generates `@llvm.umul.with.overflow.$type` intrinsic
      #[inline]
      pub(crate) const fn overflowing_umul(lhs: Self, rhs: Self) -> (Self, bool) {
        let out: Self = Self::wrapping_mul(lhs, rhs);
        let cmp: bool = rhs.zext() == 0;

        if cmp {
          (out, false)
        } else {
          // SAFETY: We already checked for division by `0`.
          (out, !Self::eq(lhs, unsafe { Self::unchecked_udiv(out, rhs) }))
        }
      }

      #[inline]
      pub(crate) const fn overflowing_sadd(lhs: Self, rhs: Self) -> (Self, bool) {
        // SAFETY: Signed addition cannot overflow the larger built-in type.
        let out: $sint = unsafe { $crate::llapi::intrinsics::unchecked_add!(lhs.sext(), rhs.sext()) };
        let cmp: bool = out > Self::SMAX || out < Self::SMIN;

        (Self::read_sint(out), cmp)
      }

      #[inline]
      pub(crate) const fn overflowing_ssub(lhs: Self, rhs: Self) -> (Self, bool) {
        // SAFETY: Signed subtraction cannot overflow the larger built-in type.
        let out: $sint = unsafe { $crate::llapi::intrinsics::unchecked_sub!(lhs.sext(), rhs.sext()) };
        let cmp: bool = out > Self::SMAX || out < Self::SMIN;

        (Self::read_sint(out), cmp)
      }

      #[inline]
      pub(crate) const fn overflowing_smul(lhs: Self, rhs: Self) -> (Self, bool) {
        let (out, cmp): ($sint, bool) = $crate::llapi::intrinsics::overflowing_mul!(lhs.sext(), rhs.sext());
        let cmp: bool = cmp || out > Self::SMAX || out < Self::SMIN;

        (Self::read_sint(out), cmp)
      }

      // -----------------------------------------------------------------------
      // Saturating Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const fn saturating_uadd(lhs: Self, rhs: Self) -> Self {
        let out: Self = Self::wrapping_add(lhs, rhs);
        let cmp: bool = Self::ucmp(out, lhs).is_lt();

        if cmp {
          return $crate::llapi::utils::Consts::UMAX;
        }

        out
      }

      #[inline]
      pub(crate) const fn saturating_usub(lhs: Self, rhs: Self) -> Self {
        if Self::ucmp(lhs, rhs).is_lt() {
          return $crate::llapi::utils::Consts::UMIN;
        }

        Self::wrapping_sub(lhs, rhs)
      }

      #[inline]
      pub(crate) const fn saturating_sadd(lhs: Self, rhs: Self) -> Self {
        // SAFETY: Signed addition cannot overflow the larger built-in type.
        let out: $sint = unsafe {
          $crate::llapi::intrinsics::unchecked_add!(lhs.sext(), rhs.sext())
        };

        if out > Self::SMAX {
          $crate::llapi::utils::Consts::SMAX
        } else if out < Self::SMIN {
          $crate::llapi::utils::Consts::SMIN
        } else {
          Self::read_sint(out)
        }
      }

      #[inline]
      pub(crate) const fn saturating_ssub(lhs: Self, rhs: Self) -> Self {
        // SAFETY: Signed addition cannot overflow the larger built-in type.
        let out: $sint = unsafe {
          $crate::llapi::intrinsics::unchecked_sub!(lhs.sext(), rhs.sext())
        };

        if out > Self::SMAX {
          $crate::llapi::utils::Consts::SMAX
        } else if out < Self::SMIN {
          $crate::llapi::utils::Consts::SMIN
        } else {
          Self::read_sint(out)
        }
      }

      // -----------------------------------------------------------------------
      // Unchecked Operations
      // -----------------------------------------------------------------------

      #[inline]
      pub(crate) const unsafe fn unchecked_uadd(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_add(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_usub(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_sub(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_umul(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_mul(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_udiv(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(unsafe {
          $crate::llapi::intrinsics::unchecked_div!(lhs.zext(), rhs.zext())
        })
      }

      #[cfg(feature = "exact_div")]
      #[inline]
      pub(crate) const unsafe fn unchecked_udiv_exact(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(unsafe {
          $crate::llapi::intrinsics::unchecked_div_exact!(lhs.zext(), rhs.zext())
        })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_urem(lhs: Self, rhs: Self) -> Self {
        Self::read_uint(unsafe {
          $crate::llapi::intrinsics::unchecked_rem!(lhs.zext(), rhs.zext())
        })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_sadd(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_add(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_ssub(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_sub(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_smul(lhs: Self, rhs: Self) -> Self {
        Self::wrapping_mul(lhs, rhs)
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_sdiv(lhs: Self, rhs: Self) -> Self {
        Self::read_sint(unsafe {
          $crate::llapi::intrinsics::unchecked_div!(lhs.sext(), rhs.sext())
        })
      }

      #[cfg(feature = "exact_div")]
      #[inline]
      pub(crate) const unsafe fn unchecked_sdiv_exact(lhs: Self, rhs: Self) -> Self {
        Self::read_sint(unsafe {
          $crate::llapi::intrinsics::unchecked_div_exact!(lhs.sext(), rhs.sext())
        })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_srem(lhs: Self, rhs: Self) -> Self {
        Self::read_sint(unsafe {
          $crate::llapi::intrinsics::unchecked_rem!(lhs.sext(), rhs.sext())
        })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_shl(int: Self, bits: u32) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        Self::read_uint(unsafe { $crate::llapi::intrinsics::unchecked_shl!(int.zext(), bits) })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_lshr(int: Self, bits: u32) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        Self::read_uint(unsafe { $crate::llapi::intrinsics::unchecked_shr!(int.zext(), bits) })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_ashr(int: Self, bits: u32) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        Self::read_sint(unsafe { $crate::llapi::intrinsics::unchecked_shr!(int.sext(), bits) })
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_fshl(lhs: Self, rhs: Self, bits: u32) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe {
          ::core::hint::assert_unchecked(bits < Self::BITS);
        }

        if bits == 0 {
          lhs
        } else {
          // SAFETY: This is guaranteed to be safe by the caller.
          unsafe {
            Self::bor(
              Self::unchecked_shl(lhs, bits),
              Self::unchecked_lshr(rhs, Self::BITS - bits),
            )
          }
        }
      }

      #[inline]
      pub(crate) const unsafe fn unchecked_fshr(lhs: Self, rhs: Self, bits: u32) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        unsafe {
          ::core::hint::assert_unchecked(bits < Self::BITS);
        }

        if bits == 0 {
          rhs
        } else {
          // SAFETY: This is guaranteed to be safe by the caller.
          unsafe {
            Self::bor(
              Self::unchecked_shl(lhs, Self::BITS - bits),
              Self::unchecked_lshr(rhs, bits),
            )
          }
        }
      }

      // -----------------------------------------------------------------------
      // Wrapping Operations
      // -----------------------------------------------------------------------

      pub(crate) const fn wrapping_add(lhs: Self, rhs: Self) -> Self {
        Self::read_uint($crate::llapi::intrinsics::wrapping_add!(lhs.zext(), rhs.zext()) & Self::UMAX)
      }

      pub(crate) const fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
        Self::read_uint($crate::llapi::intrinsics::wrapping_sub!(lhs.zext(), rhs.zext()) & Self::UMAX)
      }

      pub(crate) const fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
        Self::read_uint($crate::llapi::intrinsics::wrapping_mul!(lhs.zext(), rhs.zext()) & Self::UMAX)
      }

      // -----------------------------------------------------------------------
      // Misc. Operations
      // -----------------------------------------------------------------------

      #[cfg(feature = "disjoint_bitor")]
      #[inline]
      pub(crate) const unsafe fn disjoint_bor(lhs: Self, rhs: Self) -> Self {
        // SAFETY: This is guaranteed to be safe by the caller.
        Self::read_uint(unsafe {
          $crate::llapi::intrinsics::disjoint_bitor!(lhs.zext(), rhs.zext())
        })
      }

      #[cfg(feature = "bigint_helper_methods")]
      #[inline]
      pub(crate) const fn carrying_umul_uadd(
        lhs: Self,
        rhs: Self,
        add: Self,
        carry: Self,
      ) -> (Self, Self) {
        const M: usize = $size << 1;

        let a: [u8; M] = $crate::llapi::cast_bytes::<true, $size, { M }>(lhs.0);
        let b: [u8; M] = $crate::llapi::cast_bytes::<true, $size, { M }>(rhs.0);
        let c: [u8; M] = $crate::llapi::cast_bytes::<true, $size, { M }>(add.0);
        let d: [u8; M] = $crate::llapi::cast_bytes::<true, $size, { M }>(carry.0);

        let x: [u8; M] = $crate::llapi::wrapping_mul::<[u8; M], M>(a, b);
        let y: [u8; M] = $crate::llapi::wrapping_add::<[u8; M], M>(x, c);
        let z: [u8; M] = $crate::llapi::wrapping_add::<[u8; M], M>(y, d);

        let lo: [u8; $size] = $crate::llapi::cast_bytes::<true, { M }, $size>(z);

        let hi: [u8; $size] = $crate::llapi::cast_bytes::<true, { M }, $size>(unsafe {
          $crate::llapi::unchecked_lshr::<[u8; M], M>(z, Self::BITS)
        });

        (Self(lo), Self(hi))
      }

      #[cfg(feature = "bigint_helper_methods")]
      #[inline]
      pub(crate) const fn carrying_smul_sadd(
        lhs: Self,
        rhs: Self,
        add: Self,
        carry: Self,
      ) -> (Self, Self) {
        const M: usize = $size << 1;

        let a: [u8; M] = $crate::llapi::cast_bytes::<false, $size, { M }>(lhs.0);
        let b: [u8; M] = $crate::llapi::cast_bytes::<false, $size, { M }>(rhs.0);
        let c: [u8; M] = $crate::llapi::cast_bytes::<false, $size, { M }>(add.0);
        let d: [u8; M] = $crate::llapi::cast_bytes::<false, $size, { M }>(carry.0);

        let x: [u8; M] = $crate::llapi::wrapping_mul::<[u8; M], M>(a, b);
        let y: [u8; M] = $crate::llapi::wrapping_add::<[u8; M], M>(x, c);
        let z: [u8; M] = $crate::llapi::wrapping_add::<[u8; M], M>(y, d);

        let lo: [u8; $size] = $crate::llapi::cast_bytes::<false, { M }, $size>(z);

        let hi: [u8; $size] = $crate::llapi::cast_bytes::<false, { M }, $size>(unsafe {
          $crate::llapi::unchecked_ashr::<[u8; M], M>(z, Self::BITS)
        });

        (Self(lo), Self(hi))
      }
    }
  };
}

define_extended_integer!(I24 => T24, 3, u32, i32);
define_extended_integer!(I40 => T40, 5, u64, i64);
define_extended_integer!(I48 => T48, 6, u64, i64);
define_extended_integer!(I56 => T56, 7, u64, i64);
define_extended_integer!(I72 => T72, 9, u128, i128);
define_extended_integer!(I80 => T80, 10, u128, i128);
define_extended_integer!(I88 => T88, 11, u128, i128);
define_extended_integer!(I96 => T96, 12, u128, i128);
define_extended_integer!(I104 => T104, 13, u128, i128);
define_extended_integer!(I112 => T112, 14, u128, i128);
define_extended_integer!(I120 => T120, 15, u128, i128);
