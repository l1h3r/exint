use ::core::marker::Copy;

#[cfg(feature = "structural_match")]
use ::core::marker::StructuralPartialEq;

#[cfg(feature = "adt_const_params")]
use ::core::marker::ConstParamTy_;

use crate::types::int;
use crate::types::uint;

impl<const N: usize> Copy for int<N> {}
impl<const N: usize> Copy for uint<N> {}

#[cfg(feature = "structural_match")]
impl<const N: usize> StructuralPartialEq for int<N> {}
#[cfg(feature = "structural_match")]
impl<const N: usize> StructuralPartialEq for uint<N> {}

#[cfg(feature = "adt_const_params")]
impl<const N: usize> ConstParamTy_ for int<N> {}
#[cfg(feature = "adt_const_params")]
impl<const N: usize> ConstParamTy_ for uint<N> {}
