use crate::macros::specialize;
use crate::traits::Cast;
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
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecInspect for Int<1|2|4|8|16> {
    #[inline]
    fn ctpop(self) -> u32 {
      ::core::intrinsics::ctpop(self.ucast())
    }

    #[inline]
    fn ctlz(self) -> u32 {
      ::core::intrinsics::ctlz(self.ucast())
    }

    #[inline]
    fn cttz(self) -> u32 {
      ::core::intrinsics::cttz(self.ucast())
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
