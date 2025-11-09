macro_rules! byteorder {
  (int) => {
    $crate::types::macros::byteorder!(@core, int);
  };
  (uint) => {
    $crate::types::macros::byteorder!(@core, uint);
  };
  (@core, $name:ident) => {
    #[doc = include_doc!($name, "from_be")]
    #[must_use]
    #[inline]
    pub const fn from_be(value: Self) -> Self {
      value.to_be()
    }

    #[doc = include_doc!($name, "from_le")]
    #[must_use]
    #[inline]
    pub const fn from_le(value: Self) -> Self {
      value.to_le()
    }

    #[doc = include_doc!($name, "to_be")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_be(self) -> Self {
      #[cfg(target_endian = "big")]
      { self }

      #[cfg(target_endian = "little")]
      { self.swap_bytes() }
    }

    #[doc = include_doc!($name, "to_le")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_le(self) -> Self {
      #[cfg(target_endian = "big")]
      { self.swap_bytes() }

      #[cfg(target_endian = "little")]
      { self }
    }

    #[doc = include_doc!($name, "from_be_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
      Self::from_be(Self::from_ne_bytes(bytes))
    }

    #[doc = include_doc!($name, "from_le_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
      Self::from_le(Self::from_ne_bytes(bytes))
    }

    #[doc = include_doc!($name, "from_ne_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
      Self { bytes }
    }

    #[doc = include_doc!($name, "to_be_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N] {
      self.to_be().to_ne_bytes()
    }

    #[doc = include_doc!($name, "to_le_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
      self.to_le().to_ne_bytes()
    }

    #[doc = include_doc!($name, "to_ne_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
      self.bytes
    }
  };
  ($outer:ident, $inner:ident) => {
    #[doc = include_doc!($outer, $inner, "from_be")]
    #[must_use]
    #[inline]
    pub const fn from_be(value: Self) -> Self {
      Self($crate::types::$inner::from_be(value.0))
    }

    #[doc = include_doc!($outer, $inner, "from_le")]
    #[must_use]
    #[inline]
    pub const fn from_le(value: Self) -> Self {
      Self($crate::types::$inner::from_le(value.0))
    }

    #[doc = include_doc!($outer, $inner, "to_be")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_be(self) -> Self {
      Self(self.0.to_be())
    }

    #[doc = include_doc!($outer, $inner, "to_le")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_le(self) -> Self {
      Self(self.0.to_le())
    }

    #[doc = include_doc!($outer, $inner, "from_be_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
      Self($crate::types::$inner::from_be_bytes(bytes))
    }

    #[doc = include_doc!($outer, $inner, "from_le_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
      Self($crate::types::$inner::from_le_bytes(bytes))
    }

    #[doc = include_doc!($outer, $inner, "from_ne_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
      Self($crate::types::$inner::from_ne_bytes(bytes))
    }

    #[doc = include_doc!($outer, $inner, "to_be_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N] {
      self.0.to_be_bytes()
    }

    #[doc = include_doc!($outer, $inner, "to_le_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
      self.0.to_le_bytes()
    }

    #[doc = include_doc!($outer, $inner, "to_ne_bytes")]
    #[must_use = must_use_doc!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
      self.0.to_ne_bytes()
    }
  };
}

pub(crate) use byteorder;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use crate::tests::*;

  test!(test_be, () => {
    assert_eq!(T::from_be(T::B_1.to_be()), T::B_1);
    assert_eq!(T::from_be(T::B_2.to_be()), T::B_2);
    assert_eq!(T::from_be(T::B_3.to_be()), T::B_3);
    assert_eq!(T::from_be(T::P_0.to_be()), T::P_0);
    assert_eq!(T::from_be(T::N_1.to_be()), T::N_1);

    assert_eq!(T::from_be_bytes(T::B_1.to_be_bytes()), T::B_1);
    assert_eq!(T::from_be_bytes(T::B_2.to_be_bytes()), T::B_2);
    assert_eq!(T::from_be_bytes(T::B_3.to_be_bytes()), T::B_3);
    assert_eq!(T::from_be_bytes(T::P_0.to_be_bytes()), T::P_0);
    assert_eq!(T::from_be_bytes(T::N_1.to_be_bytes()), T::N_1);

    assert_eq!(T::P_0.to_be(), T::P_0);
    assert_eq!(T::N_1.to_be(), T::N_1);
  });

  test!(test_le, () => {
    assert_eq!(T::from_le(T::B_1.to_le()), T::B_1);
    assert_eq!(T::from_le(T::B_2.to_le()), T::B_2);
    assert_eq!(T::from_le(T::B_3.to_le()), T::B_3);
    assert_eq!(T::from_le(T::P_0.to_le()), T::P_0);
    assert_eq!(T::from_le(T::N_1.to_le()), T::N_1);

    assert_eq!(T::from_le_bytes(T::B_1.to_le_bytes()), T::B_1);
    assert_eq!(T::from_le_bytes(T::B_2.to_le_bytes()), T::B_2);
    assert_eq!(T::from_le_bytes(T::B_3.to_le_bytes()), T::B_3);
    assert_eq!(T::from_le_bytes(T::P_0.to_le_bytes()), T::P_0);
    assert_eq!(T::from_le_bytes(T::N_1.to_le_bytes()), T::N_1);

    assert_eq!(T::P_0.to_le(), T::P_0);
    assert_eq!(T::N_1.to_le(), T::N_1);
  });

  test!(test_ne, () => {
    assert_eq!(T::from_ne_bytes(T::B_1.to_ne_bytes()), T::B_1);
    assert_eq!(T::from_ne_bytes(T::B_2.to_ne_bytes()), T::B_2);
    assert_eq!(T::from_ne_bytes(T::B_3.to_ne_bytes()), T::B_3);
    assert_eq!(T::from_ne_bytes(T::P_0.to_ne_bytes()), T::P_0);
    assert_eq!(T::from_ne_bytes(T::N_1.to_ne_bytes()), T::N_1);
  });
}
