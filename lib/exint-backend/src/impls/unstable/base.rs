use ::core::cmp::Ordering;
use ::core::marker::Sized;

macro_rules! arg {
  (Self) => {
    [u8; N]
  };
  (($($tt:tt),+)) => {
    ($(arg!($tt)),+)
  };
  ($type:ty) => {
    $type
  };
}

macro_rules! define_traits {
  (
    @docstring = $docstring:literal;
    @base_name = $base_name:ident $(: $base_bound:ident)?;
    @spec_name = $spec_name:ident $(: $spec_bound:ident)?;
    @functions = ($(
      $(#[$meta:meta])*
      $function:ident: $($safety:ident)? |$($name:ident: $type:tt),+| $(-> $return:tt)?;
    )+);
  ) => {
    #[doc = $docstring]
    #[const_trait]
    pub(crate) trait $base_name $(: $base_bound)? {
      $(
        $(#[$meta])*
        $($safety)? fn $function($($name: $type),+) $(-> $return)?;
      )+
    }

    #[doc = $docstring]
    #[const_trait]
    pub(crate) trait $spec_name $(: $spec_bound)? {
      $(
        $(#[$meta])*
        $($safety)? fn $function($($name: $type),+) $(-> $return)?;
      )+
    }

    impl<const N: usize> const $base_name for [u8; N] {
      $(
        #[allow(unsafe_op_in_unsafe_fn, reason = "Non-public macro-generated code")]
        #[inline(always)]
        $(#[$meta])*
        $($safety)? fn $function($($name: $type),+) $(-> $return)? {
          $crate::impls::fallback::$function($($name),+)
        }
      )+
    }

    #[cfg(feature = "min_specialization")]
    impl<T: ~const $base_name> const $spec_name for T {
      $(
        #[allow(unsafe_op_in_unsafe_fn, reason = "Non-public macro-generated code")]
        #[inline(always)]
        $(#[$meta])*
        default $($safety)? fn $function($($name: $type),+) $(-> $return)? {
          <T as $base_name>::$function($($name),+)
        }
      )+
    }

    #[cfg(not(feature = "min_specialization"))]
    impl<T: ~const $base_name> const $spec_name for T {
      $(
        #[allow(unsafe_op_in_unsafe_fn, reason = "Non-public macro-generated code")]
        #[inline(always)]
        $(#[$meta])*
        $($safety)? fn $function($($name: $type),+) $(-> $return)? {
          <T as $base_name>::$function($($name),+)
        }
      )+
    }

    $(
      #[allow(unsafe_op_in_unsafe_fn, reason = "Non-public macro-generated code")]
      #[inline(always)]
      $(#[$meta])*
      pub(crate) const $($safety)? fn $function<const N: usize>($($name: arg!($type)),+) $(-> arg!($return))? {
        <[u8; N] as $spec_name>::$function($($name),+)
      }
    )+
  };
}

define_traits! {
  @docstring = "Supporting trait for specialized operations.";
  @base_name = BaseCore: Sized;
  @spec_name = SpecCore: Sized;
  @functions = (
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------
    band: |lhs: Self, rhs: Self| -> Self;
    bor:  |lhs: Self, rhs: Self| -> Self;
    bxor: |lhs: Self, rhs: Self| -> Self;
    bnot: |integer: Self| -> Self;
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------
    eq: |lhs: Self, rhs: Self| -> bool;
    // -------------------------------------------------------------------------
    // Bit Conversion Operation
    // -------------------------------------------------------------------------
    swap1: |integer: Self| -> Self;
    swap8: |integer: Self| -> Self;
    rotl:  |integer: Self, bits: u32| -> Self;
    rotr:  |integer: Self, bits: u32| -> Self;
    // -------------------------------------------------------------------------
    // Bit Inspection Operations
    // -------------------------------------------------------------------------
    ctpop: |integer: Self| -> u32;
    ctlz:  |integer: Self| -> u32;
    cttz:  |integer: Self| -> u32;
    ctlz_nonzero: unsafe |integer: Self| -> u32;
    cttz_nonzero: unsafe |integer: Self| -> u32;
    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------
    unchecked_shl: unsafe |integer: Self, bits: u32| -> Self;
    // -------------------------------------------------------------------------
    // Wrapping Operations
    // -------------------------------------------------------------------------
    wrapping_add: |lhs: Self, rhs: Self| -> Self;
    wrapping_sub: |lhs: Self, rhs: Self| -> Self;
    wrapping_mul: |lhs: Self, rhs: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for specialized operations.";
  @base_name = BaseSint: SpecCore;
  @spec_name = SpecSint: SpecCore;
  @functions = (
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------
    scmp: |lhs: Self, rhs: Self| -> Ordering;
    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------
    overflowing_sadd: |lhs: Self, rhs: Self| -> (Self, bool);
    overflowing_ssub: |lhs: Self, rhs: Self| -> (Self, bool);
    overflowing_smul: |lhs: Self, rhs: Self| -> (Self, bool);
    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------
    saturating_sadd: |lhs: Self, rhs: Self| -> Self;
    saturating_ssub: |lhs: Self, rhs: Self| -> Self;
    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------
    unchecked_sadd: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_ssub: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_smul: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_sdiv: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_srem: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_ashr: unsafe |integer: Self, bits: u32| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for specialized operations.";
  @base_name = BaseUint: SpecCore;
  @spec_name = SpecUint: SpecCore;
  @functions = (
    // -------------------------------------------------------------------------
    // Bitwise Operations
    // -------------------------------------------------------------------------
    #[cfg(feature = "disjoint_bitor")]
    disjoint_bor: unsafe |lhs: Self, rhs: Self| -> Self;
    // -------------------------------------------------------------------------
    // Comparison Operations
    // -------------------------------------------------------------------------
    ucmp: |lhs: Self, rhs: Self| -> Ordering;
    // -------------------------------------------------------------------------
    // Overflowing Operations
    // -------------------------------------------------------------------------
    overflowing_uadd: |lhs: Self, rhs: Self| -> (Self, bool);
    overflowing_usub: |lhs: Self, rhs: Self| -> (Self, bool);
    overflowing_umul: |lhs: Self, rhs: Self| -> (Self, bool);
    // -------------------------------------------------------------------------
    // Saturating Operations
    // -------------------------------------------------------------------------
    saturating_uadd: |lhs: Self, rhs: Self| -> Self;
    saturating_usub: |lhs: Self, rhs: Self| -> Self;
    // -------------------------------------------------------------------------
    // Unchecked Operations
    // -------------------------------------------------------------------------
    unchecked_uadd: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_usub: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_umul: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_udiv: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_urem: unsafe |lhs: Self, rhs: Self| -> Self;
    unchecked_lshr: unsafe |integer: Self, bits: u32| -> Self;
  );
}
