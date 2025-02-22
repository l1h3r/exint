pub(crate) trait TryConvert<T> {
  type Error;

  fn try_convert(self) -> ::core::result::Result<T, Self::Error>;
}

macro_rules! implement_lib {
  ($name:ident) => {
    impl<const N: usize> TryConvert<u32> for $crate::$name<N> {
      type Error = $crate::error::TryFromIntError;

      #[inline]
      fn try_convert(self) -> ::core::result::Result<u32, Self::Error> {
        let min: Self = $crate::utils::Cast::cast(u32::MIN);
        let max: Self = $crate::utils::Cast::cast(u32::MAX);

        if self.const_lt(&min) || self.const_gt(&max) {
          ::core::result::Result::Err($crate::error::TryFromIntError::new())
        } else {
          ::core::result::Result::Ok(self.into_u32())
        }
      }
    }
  };
}

implement_lib!(int);
implement_lib!(uint);

macro_rules! implement_std {
  ($type:ty) => {
    impl TryConvert<u32> for $type {
      type Error = <$type as ::core::convert::TryInto<u32>>::Error;

      #[inline]
      fn try_convert(self) -> ::core::result::Result<u32, Self::Error> {
        ::core::convert::TryInto::try_into(self)
      }
    }
  };
  ($($type:ty)+) => {
    $(
      implement_std!($type);
    )+
  };
}

implement_std!(i8 i16 i32 i64 i128 isize);
implement_std!(u8 u16 u32 u64 u128 usize);
