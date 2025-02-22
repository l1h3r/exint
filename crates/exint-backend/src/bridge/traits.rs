//! Supporting traits for integer operations, for internal use only.

use ::core::cmp::Ordering;
use ::core::marker::Sized;

macro_rules! define_traits {
  (
    @docstring = $docstring:literal;
    @base_name = impl $base_name:ident $(: $base_bound:ident)?;
    @spec_name = impl $spec_name:ident $(: $spec_bound:ident)?;
    @functions = ($(
      $function:ident: $($safety:ident)? |self $(, $arg_name:ident: $arg_type:ty)*| -> $retval:ty;
    )+);
  ) => {
    #[doc = $docstring]
    #[const_trait]
    pub(crate) trait $base_name $(: $base_bound)? {
      $(
        $($safety)? fn $function(self $(, $arg_name: $arg_type)*) -> $retval;
      )+
    }

    #[doc = $docstring]
    #[const_trait]
    pub(crate) trait $spec_name $(: $spec_bound)? {
      $(
        $($safety)? fn $function(self $(, $arg_name: $arg_type)*) -> $retval;
      )+
    }

    impl<T> const $spec_name for T
    where
      T: ~const $base_name,
    {
      $(
        #[allow(unsafe_op_in_unsafe_fn)]
        #[inline]
        default $($safety)? fn $function(self $(, $arg_name: $arg_type)*) -> $retval {
          <T as $base_name>::$function(self $(, $arg_name)*)
        }
      )+
    }
  };
}

define_traits! {
  @docstring = "Supporting trait for bitwise operations.";
  @base_name = impl BaseBitwise;
  @spec_name = impl SpecBitwise;
  @functions = (
    and: |self, other: Self| -> Self;
    or:  |self, other: Self| -> Self;
    xor: |self, other: Self| -> Self;
    not: |self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for comparison operations.";
  @base_name = impl BaseCompare;
  @spec_name = impl SpecCompare;
  @functions = (
    eq:   |self, other: Self| -> bool;
    ucmp: |self, other: Self| -> Ordering;
    scmp: |self, other: Self| -> Ordering;
  );
}

define_traits! {
  @docstring = "Supporting trait for conversion operations.";
  @base_name = impl BaseConvert;
  @spec_name = impl SpecConvert;
  @functions = (
    swap1: |self| -> Self;
    swap8: |self| -> Self;
    rotl:  |self, bits: u32| -> Self;
    rotr:  |self, bits: u32| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for inspection operations.";
  @base_name = impl BaseInspect;
  @spec_name = impl SpecInspect;
  @functions = (
    ctpop: |self| -> u32;
    ctlz:  |self| -> u32;
    cttz:  |self| -> u32;
    ctlz_nonzero: unsafe |self| -> u32;
    cttz_nonzero: unsafe |self| -> u32;
  );
}

define_traits! {
  @docstring = "Supporting trait for bitwise shift operations.";
  @base_name = impl BaseShift;
  @spec_name = impl SpecShift;
  @functions = (
    shl:  unsafe |self, bits: u32| -> Self;
    lshr: unsafe |self, bits: u32| -> Self;
    ashr: unsafe |self, bits: u32| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for unsigned addition operations.";
  @base_name = impl BaseUadd: Sized;
  @spec_name = impl SpecUadd: BaseUadd;
  @functions = (
    oadd: |self, other: Self| -> (Self, bool);
    sadd: |self, other: Self| -> Self;
    wadd: |self, other: Self| -> Self;
    uadd: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for signed addition operations.";
  @base_name = impl BaseSadd: SpecUadd;
  @spec_name = impl SpecSadd: SpecUadd;
  @functions = (
    oadd: |self, other: Self| -> (Self, bool);
    sadd: |self, other: Self| -> Self;
    wadd: |self, other: Self| -> Self;
    uadd: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for unsigned subtraction operations.";
  @base_name = impl BaseUsub: Sized;
  @spec_name = impl SpecUsub: BaseUsub;
  @functions = (
    osub: |self, other: Self| -> (Self, bool);
    ssub: |self, other: Self| -> Self;
    wsub: |self, other: Self| -> Self;
    usub: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for signed subtraction operations.";
  @base_name = impl BaseSsub: SpecUsub;
  @spec_name = impl SpecSsub: SpecUsub;
  @functions = (
    osub: |self, other: Self| -> (Self, bool);
    ssub: |self, other: Self| -> Self;
    wsub: |self, other: Self| -> Self;
    usub: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for unsigned multiplication operations.";
  @base_name = impl BaseUmul: Sized;
  @spec_name = impl SpecUmul: BaseUmul;
  @functions = (
    omul: |self, other: Self| -> (Self, bool);
    wmul: |self, other: Self| -> Self;
    umul: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for signed multiplication operations.";
  @base_name = impl BaseSmul: SpecUmul;
  @spec_name = impl SpecSmul: SpecUmul;
  @functions = (
    omul: |self, other: Self| -> (Self, bool);
    wmul: |self, other: Self| -> Self;
    umul: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for unsigned division operations.";
  @base_name = impl BaseUdiv: Sized;
  @spec_name = impl SpecUdiv: BaseUdiv;
  @functions = (
    udiv: unsafe |self, other: Self| -> Self;
    urem: unsafe |self, other: Self| -> Self;
    ediv: unsafe |self, other: Self| -> Self;
  );
}

define_traits! {
  @docstring = "Supporting trait for signed division operations.";
  @base_name = impl BaseSdiv: Sized;
  @spec_name = impl SpecSdiv: BaseSdiv;
  @functions = (
    udiv: unsafe |self, other: Self| -> Self;
    urem: unsafe |self, other: Self| -> Self;
    ediv: unsafe |self, other: Self| -> Self;
  );
}
