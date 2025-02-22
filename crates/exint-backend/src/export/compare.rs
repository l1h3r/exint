use crate::macros::specialize;
use crate::traits::Cast;
use crate::types::Int;

// -----------------------------------------------------------------------------
// Trait Definition
// -----------------------------------------------------------------------------

/// Supporting trait for specialized intrinsics.
#[const_trait]
pub(crate) trait SpecCompare {
  fn eq(self, other: Self) -> bool;
  fn ucmp(self, other: Self) -> ::core::cmp::Ordering;
  fn scmp(self, other: Self) -> ::core::cmp::Ordering;
}

// -----------------------------------------------------------------------------
// Implementation - Default
// -----------------------------------------------------------------------------

impl<const S: usize> const SpecCompare for Int<S> {
  default fn eq(self, other: Self) -> bool {
    ::core::panic!("SpecCompare::eq")
  }

  default fn ucmp(self, other: Self) -> ::core::cmp::Ordering {
    ::core::panic!("SpecCompare::ucmp")
  }

  default fn scmp(self, other: Self) -> ::core::cmp::Ordering {
    ::core::panic!("SpecCompare::scmp")
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for 'std' sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecCompare for Int<1|2|4|8|16> {
    #[inline]
    fn eq(self, other: Self) -> bool {
      // SAFETY:
      //   - integer bytes **are not** uninitialized
      //   - integer types **do not** have padding
      unsafe { ::core::intrinsics::raw_eq(&self, &other) }
    }

    #[inline]
    fn ucmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.ucast(), other.ucast())
    }

    #[inline]
    fn scmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.scast(), other.scast())
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------
