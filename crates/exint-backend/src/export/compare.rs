use crate::macros::specialize;
use crate::traits::Cast;
use crate::traits::Exts;
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
    // SAFETY:
    //   - integer bytes **are not** uninitialized
    //   - integer types **do not** have padding
    unsafe { ::core::intrinsics::raw_eq(&self, &other) }
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
    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.ucast(), other.ucast())
    }

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.scast(), other.scast())
    }
  }
}

// -----------------------------------------------------------------------------
// Implementation - Specialization for common sizes
// -----------------------------------------------------------------------------

specialize! {
  impl SpecCompare for Int<3|5|6|7|9|10|11|12|13|14|15> {
    // LLVM generates `icmp ult $type` and `icmp ne $type` instructions
    #[inline]
    fn ucmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.zext(), other.zext())
    }

    // LLVM generates `icmp slt $type` and `icmp ne $type` instructions
    #[inline]
    fn scmp(self, other: Self) -> ::core::cmp::Ordering {
      ::core::intrinsics::three_way_compare(self.sext(), other.sext())
    }
  }
}
