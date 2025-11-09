#[cfg(feature = "core_intrinsics")]
mod enabled {
  // ---------------------------------------------------------------------------
  // Bit Conversion Operations
  // ---------------------------------------------------------------------------

  macro_rules! bitreverse {
    ($int:expr) => {
      ::core::intrinsics::bitreverse($int)
    };
  }

  macro_rules! bswap {
    ($int:expr) => {
      ::core::intrinsics::bswap($int)
    };
  }

  macro_rules! rotate_left {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::rotate_left($lhs, $rhs)
    };
  }

  macro_rules! rotate_right {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::rotate_right($lhs, $rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Bit Inspection Operations
  // ---------------------------------------------------------------------------

  macro_rules! ctpop {
    ($int:expr) => {
      ::core::intrinsics::ctpop($int)
    };
  }

  macro_rules! ctlz {
    ($int:expr) => {
      ::core::intrinsics::ctlz($int)
    };
  }

  macro_rules! cttz {
    ($int:expr) => {
      ::core::intrinsics::cttz($int)
    };
  }

  macro_rules! ctlz_nonzero {
    ($int:expr) => {
      ::core::intrinsics::ctlz_nonzero($int)
    };
  }

  macro_rules! cttz_nonzero {
    ($int:expr) => {
      ::core::intrinsics::cttz_nonzero($int)
    };
  }

  // ---------------------------------------------------------------------------
  // Overflowing Operations
  // ---------------------------------------------------------------------------

  macro_rules! overflowing_add {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::add_with_overflow($lhs, $rhs)
    };
  }

  macro_rules! overflowing_sub {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::sub_with_overflow($lhs, $rhs)
    };
  }

  macro_rules! overflowing_mul {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::mul_with_overflow($lhs, $rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Saturating Operations
  // ---------------------------------------------------------------------------

  macro_rules! saturating_add {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::saturating_add($lhs, $rhs)
    };
  }

  macro_rules! saturating_sub {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::saturating_sub($lhs, $rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Unchecked Operations
  // ---------------------------------------------------------------------------

  macro_rules! unchecked_add {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_add($lhs, $rhs)
    };
  }

  macro_rules! unchecked_sub {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_sub($lhs, $rhs)
    };
  }

  macro_rules! unchecked_mul {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_mul($lhs, $rhs)
    };
  }

  macro_rules! unchecked_div {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_div($lhs, $rhs)
    };
  }

  macro_rules! unchecked_rem {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_rem($lhs, $rhs)
    };
  }

  #[cfg(feature = "exact_div")]
  macro_rules! unchecked_div_exact {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::exact_div($lhs, $rhs)
    };
  }

  macro_rules! unchecked_shl {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_shl($lhs, $rhs)
    };
  }

  macro_rules! unchecked_shr {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::unchecked_shr($lhs, $rhs)
    };
  }

  #[cfg(feature = "funnel_shifts")]
  macro_rules! unchecked_funnel_shl {
    ($lhs:expr, $rhs:expr, $bits:expr) => {
      ::core::intrinsics::unchecked_funnel_shl($lhs, $rhs, $bits)
    };
  }

  #[cfg(feature = "funnel_shifts")]
  macro_rules! unchecked_funnel_shr {
    ($lhs:expr, $rhs:expr, $bits:expr) => {
      ::core::intrinsics::unchecked_funnel_shr($lhs, $rhs, $bits)
    };
  }

  // ---------------------------------------------------------------------------
  // Wrapping Operations
  // ---------------------------------------------------------------------------

  macro_rules! wrapping_add {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::wrapping_add($lhs, $rhs)
    };
  }

  macro_rules! wrapping_sub {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::wrapping_sub($lhs, $rhs)
    };
  }

  macro_rules! wrapping_mul {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::wrapping_mul($lhs, $rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Misc. Operations
  // ---------------------------------------------------------------------------

  #[cfg(feature = "disjoint_bitor")]
  macro_rules! disjoint_bitor {
    ($lhs:expr, $rhs:expr) => {
      ::core::intrinsics::disjoint_bitor($lhs, $rhs)
    };
  }

  #[cfg(feature = "bigint_helper_methods")]
  macro_rules! carrying_mul_add {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
      ::core::intrinsics::carrying_mul_add($a, $b, $c, $d)
    };
  }

  pub(crate) use bitreverse;
  pub(crate) use bswap;
  pub(crate) use rotate_left;
  pub(crate) use rotate_right;
  pub(crate) use ctpop;
  pub(crate) use ctlz;
  pub(crate) use cttz;
  pub(crate) use ctlz_nonzero;
  pub(crate) use cttz_nonzero;
  pub(crate) use overflowing_add;
  pub(crate) use overflowing_sub;
  pub(crate) use overflowing_mul;
  pub(crate) use saturating_add;
  pub(crate) use saturating_sub;
  pub(crate) use unchecked_add;
  pub(crate) use unchecked_sub;
  pub(crate) use unchecked_mul;
  pub(crate) use unchecked_div;
  pub(crate) use unchecked_rem;
  pub(crate) use unchecked_shl;
  pub(crate) use unchecked_shr;
  pub(crate) use wrapping_add;
  pub(crate) use wrapping_sub;
  pub(crate) use wrapping_mul;

  #[cfg(feature = "exact_div")]
  pub(crate) use unchecked_div_exact;

  #[cfg(feature = "funnel_shifts")]
  pub(crate) use unchecked_funnel_shl;

  #[cfg(feature = "funnel_shifts")]
  pub(crate) use unchecked_funnel_shr;

  #[cfg(feature = "disjoint_bitor")]
  pub(crate) use disjoint_bitor;

  #[cfg(feature = "bigint_helper_methods")]
  pub(crate) use carrying_mul_add;
}

#[cfg(not(feature = "core_intrinsics"))]
mod default {
  // ---------------------------------------------------------------------------
  // Bit Conversion Operations
  // ---------------------------------------------------------------------------

  macro_rules! bitreverse {
    ($int:expr) => {
      $int.reverse_bits()
    };
  }

  macro_rules! bswap {
    ($int:expr) => {
      $int.swap_bytes()
    };
  }

  macro_rules! rotate_left {
    ($lhs:expr, $rhs:expr) => {
      $lhs.rotate_left($rhs)
    };
  }

  macro_rules! rotate_right {
    ($lhs:expr, $rhs:expr) => {
      $lhs.rotate_right($rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Bit Inspection Operations
  // ---------------------------------------------------------------------------

  macro_rules! ctpop {
    ($int:expr) => {
      $int.count_ones()
    };
  }

  macro_rules! ctlz {
    ($int:expr) => {
      $int.leading_zeros()
    };
  }

  macro_rules! cttz {
    ($int:expr) => {
      $int.trailing_zeros()
    };
  }

  macro_rules! ctlz_nonzero {
    ($int:expr) => {
      ::core::num::NonZero::new_unchecked($int).leading_zeros()
    };
  }

  macro_rules! cttz_nonzero {
    ($int:expr) => {
      ::core::num::NonZero::new_unchecked($int).trailing_zeros()
    };
  }

  // ---------------------------------------------------------------------------
  // Overflowing Operations
  // ---------------------------------------------------------------------------

  macro_rules! overflowing_add {
    ($lhs:expr, $rhs:expr) => {
      $lhs.overflowing_add($rhs)
    };
  }

  macro_rules! overflowing_sub {
    ($lhs:expr, $rhs:expr) => {
      $lhs.overflowing_sub($rhs)
    };
  }

  macro_rules! overflowing_mul {
    ($lhs:expr, $rhs:expr) => {
      $lhs.overflowing_mul($rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Saturating Operations
  // ---------------------------------------------------------------------------

  macro_rules! saturating_add {
    ($lhs:expr, $rhs:expr) => {
      $lhs.saturating_add($rhs)
    };
  }

  macro_rules! saturating_sub {
    ($lhs:expr, $rhs:expr) => {
      $lhs.saturating_sub($rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Unchecked Operations
  // ---------------------------------------------------------------------------

  macro_rules! unchecked_add {
    ($lhs:expr, $rhs:expr) => {
      $lhs.unchecked_add($rhs)
    };
  }

  macro_rules! unchecked_sub {
    ($lhs:expr, $rhs:expr) => {
      $lhs.unchecked_sub($rhs)
    };
  }

  macro_rules! unchecked_mul {
    ($lhs:expr, $rhs:expr) => {
      $lhs.unchecked_mul($rhs)
    };
  }

  macro_rules! unchecked_div {
    ($lhs:expr, $rhs:expr) => {
      $lhs.checked_div($rhs).unwrap_unchecked()
    };
  }

  macro_rules! unchecked_rem {
    ($lhs:expr, $rhs:expr) => {
      $lhs.checked_div($rhs).unwrap_unchecked()
    };
  }

  #[cfg(feature = "exact_div")]
  macro_rules! unchecked_div_exact {
    ($lhs:expr, $rhs:expr) => {
      $lhs.unchecked_exact_div($rhs)
    };
  }

  macro_rules! unchecked_shl {
    ($lhs:expr, $rhs:expr) => {
      $lhs.checked_shl($rhs).unwrap_unchecked()
    };
  }

  macro_rules! unchecked_shr {
    ($lhs:expr, $rhs:expr) => {
      $lhs.checked_shr($rhs).unwrap_unchecked()
    };
  }

  #[cfg(feature = "funnel_shifts")]
  macro_rules! unchecked_funnel_shl {
    ($lhs:expr, $rhs:expr, $bits:expr) => {{
      let max: u32 = 8 * ::core::mem::size_of_val(&$lhs) as u32;

      #[expect(unused_unsafe, reason = "expecting parent unsafe block")]
      unsafe {
        ::core::hint::assert_unchecked($bits < max);
      }

      if $bits == 0 {
        $lhs
      } else {
        $crate::llapi::intrinsics::unchecked_shl!($lhs, $bits)
          | $crate::llapi::intrinsics::unchecked_shr!($rhs, max - $bits)
      }
    }};
  }

  #[cfg(feature = "funnel_shifts")]
  macro_rules! unchecked_funnel_shr {
    ($lhs:expr, $rhs:expr, $bits:expr) => {{
      let max: u32 = 8 * ::core::mem::size_of_val(&$lhs) as u32;

      #[expect(unused_unsafe, reason = "expecting parent unsafe block")]
      unsafe {
        ::core::hint::assert_unchecked($bits < max);
      }

      if $bits == 0 {
        $rhs
      } else {
        $crate::llapi::intrinsics::unchecked_shl!($lhs, max - $bits)
          | $crate::llapi::intrinsics::unchecked_shr!($rhs, $bits)
      }
    }};
  }

  // ---------------------------------------------------------------------------
  // Wrapping Operations
  // ---------------------------------------------------------------------------

  macro_rules! wrapping_add {
    ($lhs:expr, $rhs:expr) => {
      $lhs.wrapping_add($rhs)
    };
  }

  macro_rules! wrapping_sub {
    ($lhs:expr, $rhs:expr) => {
      $lhs.wrapping_sub($rhs)
    };
  }

  macro_rules! wrapping_mul {
    ($lhs:expr, $rhs:expr) => {
      $lhs.wrapping_mul($rhs)
    };
  }

  // ---------------------------------------------------------------------------
  // Misc. Operations
  // ---------------------------------------------------------------------------

  #[cfg(feature = "disjoint_bitor")]
  macro_rules! disjoint_bitor {
    ($lhs:expr, $rhs:expr) => {
      $lhs.unchecked_disjoint_bitor($rhs)
    };
  }

  #[cfg(feature = "bigint_helper_methods")]
  macro_rules! carrying_mul_add {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
      $a.carrying_mul_add($b, $c, $d)
    };
  }

  pub(crate) use bitreverse;
  pub(crate) use bswap;
  pub(crate) use rotate_left;
  pub(crate) use rotate_right;
  pub(crate) use ctpop;
  pub(crate) use ctlz;
  pub(crate) use cttz;
  pub(crate) use ctlz_nonzero;
  pub(crate) use cttz_nonzero;
  pub(crate) use overflowing_add;
  pub(crate) use overflowing_sub;
  pub(crate) use overflowing_mul;
  pub(crate) use saturating_add;
  pub(crate) use saturating_sub;
  pub(crate) use unchecked_add;
  pub(crate) use unchecked_sub;
  pub(crate) use unchecked_mul;
  pub(crate) use unchecked_div;
  pub(crate) use unchecked_rem;
  pub(crate) use unchecked_shl;
  pub(crate) use unchecked_shr;
  pub(crate) use wrapping_add;
  pub(crate) use wrapping_sub;
  pub(crate) use wrapping_mul;

  #[cfg(feature = "exact_div")]
  pub(crate) use unchecked_div_exact;

  #[cfg(feature = "funnel_shifts")]
  pub(crate) use unchecked_funnel_shl;

  #[cfg(feature = "funnel_shifts")]
  pub(crate) use unchecked_funnel_shr;

  #[cfg(feature = "disjoint_bitor")]
  pub(crate) use disjoint_bitor;

  #[cfg(feature = "bigint_helper_methods")]
  pub(crate) use carrying_mul_add;
}

macro_rules! three_way_compare {
  ($lhs:expr, $rhs:expr) => {
    if $lhs < $rhs {
      ::core::cmp::Ordering::Less
    } else if $lhs == $rhs {
      ::core::cmp::Ordering::Equal
    } else {
      ::core::cmp::Ordering::Greater
    }
  };
}

#[cfg(feature = "core_intrinsics")]
pub(crate) use self::enabled::*;

#[cfg(not(feature = "core_intrinsics"))]
pub(crate) use self::default::*;

pub(crate) use three_way_compare;
