use crate::export::compare::SpecCompare;
use crate::export::convert::SpecConvert;
use crate::macros::specialize;
use crate::traits::Cast;
use crate::traits::Exts;
use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecInspect {
  fn ctpop(self) -> u32;
  fn ctlz(self) -> u32;
  fn cttz(self) -> u32;
  unsafe fn ctlz_nonzero(self) -> u32;
  unsafe fn cttz_nonzero(self) -> u32;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecInspect for Int<S> {
  default fn ctpop(self) -> u32 {
    ::core::panic!("SpecInspect::ctpop")
  }

  default fn ctlz(self) -> u32 {
    ::core::panic!("SpecInspect::ctlz")
  }

  default fn cttz(self) -> u32 {
    ::core::panic!("SpecInspect::cttz")
  }

  default unsafe fn ctlz_nonzero(self) -> u32 {
    // Inform the optimizer that this value is **not** zero.
    //
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe {
      ::core::intrinsics::assume(!SpecCompare::eq(self, [0; S]));
    }

    SpecInspect::ctlz(self)
  }

  default unsafe fn cttz_nonzero(self) -> u32 {
    // Inform the optimizer that this value is **not** zero.
    //
    // SAFETY: This is guaranteed to be safe by the caller.
    unsafe {
      ::core::intrinsics::assume(!SpecCompare::eq(self, [0; S]));
    }

    SpecInspect::cttz(self)
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecInspect for Int<1|2|4|8|16> {
    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(self) -> u32 {
      ::core::intrinsics::ctpop(self.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(self) -> u32 {
      ::core::intrinsics::ctlz(self.ucast())
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(self) -> u32 {
      ::core::intrinsics::cttz(self.ucast())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn ctlz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::ctlz_nonzero(self.ucast())
      }
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic with `nonzero` flag
    #[inline]
    unsafe fn cttz_nonzero(self) -> u32 {
      // SAFETY: This is guaranteed to be safe by the caller.
      unsafe {
        ::core::intrinsics::cttz_nonzero(self.ucast())
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecInspect for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `@llvm.ctpop.$type` intrinsic
    #[inline]
    fn ctpop(self) -> u32 {
      ::core::intrinsics::ctpop(self.zext())
    }

    // LLVM generates `@llvm.ctlz.$type` intrinsic
    #[inline]
    fn ctlz(self) -> u32 {
      SpecConvert::swap1(self).cttz()
    }

    // LLVM generates `@llvm.cttz.$type` intrinsic
    #[inline]
    fn cttz(self) -> u32 {
      if self.zext() == 0 {
        return Self::BITS;
      }

      ::core::intrinsics::cttz(self.zext())
    }
  }
}
