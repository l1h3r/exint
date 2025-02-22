//! Exint Integer

#![feature(doc_cfg)]
#![cfg_attr(feature = "random",       feature(random))]
#![cfg_attr(feature = "step_trait",   feature(step_trait))]
#![cfg_attr(feature = "trusted_step", feature(min_specialization))]
#![cfg_attr(feature = "trusted_step", feature(trusted_step))]
#![no_std]

mod macros;
mod traits;
mod types;

// TODO
pub mod errors {
  pub type ParseIntError = ();
}

// TODO: Move to exint-backend
pub mod intrinsics {
  pub const fn ctpop<T: Copy, const S: usize>(integer: T) -> u32 {
    panic!("intrinsics::ctpop")
  }

  pub const fn ctlz<T: Copy, const S: usize>(integer: T) -> u32 {
    panic!("intrinsics::ctlz")
  }

  pub const fn cttz<T: Copy, const S: usize>(integer: T) -> u32 {
    panic!("intrinsics::cttz")
  }

  pub const fn swap1<T: Copy, const S: usize>(integer: T) -> T {
    panic!("intrinsics::swap1")
  }

  pub const fn swap8<T: Copy, const S: usize>(integer: T) -> T {
    panic!("intrinsics::swap8")
  }

  pub const fn rotl<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
    panic!("intrinsics::rotl")
  }

  pub const fn rotr<T: Copy, const S: usize>(integer: T, bits: u32) -> T {
    panic!("intrinsics::rotr")
  }
}

pub use self::types::sint::int;
pub use self::types::uint::uint;

pub mod primitive {
  //! Integer type-aliases

  macro_rules! define {
    ($export:ident as $name:ident<$size:literal>, $doc:literal) => {
      #[doc = $doc]
      #[allow(non_camel_case_types)]
      pub type $export = $crate::$name<$size>;
    };
  }

  define!(u8   as uint<1>,  "The 8-bit unsigned integer type.");
  define!(u16  as uint<2>,  "The 16-bit unsigned integer type.");
  define!(u24  as uint<3>,  "The 24-bit unsigned integer type.");
  define!(u32  as uint<4>,  "The 32-bit unsigned integer type.");
  define!(u40  as uint<5>,  "The 40-bit unsigned integer type.");
  define!(u48  as uint<6>,  "The 48-bit unsigned integer type.");
  define!(u56  as uint<7>,  "The 56-bit unsigned integer type.");
  define!(u64  as uint<8>,  "The 64-bit unsigned integer type.");
  define!(u72  as uint<9>,  "The 72-bit unsigned integer type.");
  define!(u80  as uint<10>, "The 80-bit unsigned integer type.");
  define!(u88  as uint<11>, "The 88-bit unsigned integer type.");
  define!(u96  as uint<12>, "The 96-bit unsigned integer type.");
  define!(u104 as uint<13>, "The 104-bit unsigned integer type.");
  define!(u112 as uint<14>, "The 112-bit unsigned integer type.");
  define!(u120 as uint<15>, "The 120-bit unsigned integer type.");
  define!(u128 as uint<16>, "The 128-bit unsigned integer type.");
  define!(u256 as uint<32>, "The 256-bit unsigned integer type.");
  define!(u512 as uint<64>, "The 512-bit unsigned integer type.");

  #[cfg(target_pointer_width = "16")]
  define!(usize as uint<2>, "The pointer-sized unsigned integer type.");
  #[cfg(target_pointer_width = "32")]
  define!(usize as uint<4>, "The pointer-sized unsigned integer type.");
  #[cfg(target_pointer_width = "64")]
  define!(usize as uint<8>, "The pointer-sized unsigned integer type.");

  define!(i8   as int<1>,  "The 8-bit signed integer type.");
  define!(i16  as int<2>,  "The 16-bit signed integer type.");
  define!(i24  as int<3>,  "The 24-bit signed integer type.");
  define!(i32  as int<4>,  "The 32-bit signed integer type.");
  define!(i40  as int<5>,  "The 40-bit signed integer type.");
  define!(i48  as int<6>,  "The 48-bit signed integer type.");
  define!(i56  as int<7>,  "The 56-bit signed integer type.");
  define!(i64  as int<8>,  "The 64-bit signed integer type.");
  define!(i72  as int<9>,  "The 72-bit signed integer type.");
  define!(i80  as int<10>, "The 80-bit signed integer type.");
  define!(i88  as int<11>, "The 88-bit signed integer type.");
  define!(i96  as int<12>, "The 96-bit signed integer type.");
  define!(i104 as int<13>, "The 104-bit signed integer type.");
  define!(i112 as int<14>, "The 112-bit signed integer type.");
  define!(i120 as int<15>, "The 120-bit signed integer type.");
  define!(i128 as int<16>, "The 128-bit signed integer type.");
  define!(i256 as int<32>, "The 256-bit signed integer type.");
  define!(i512 as int<64>, "The 512-bit signed integer type.");

  #[cfg(target_pointer_width = "16")]
  define!(isize as int<2>, "The pointer-sized signed integer type.");
  #[cfg(target_pointer_width = "32")]
  define!(isize as int<4>, "The pointer-sized signed integer type.");
  #[cfg(target_pointer_width = "64")]
  define!(isize as int<8>, "The pointer-sized signed integer type.");
}
