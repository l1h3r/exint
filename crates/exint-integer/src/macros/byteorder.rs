macro_rules! byteorder {
  (core, $_name:ident, $_uint:expr) => {
    #[must_use]
    #[inline]
    pub const fn from_be(x: Self) -> Self {
      x.to_be()
    }

    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; S]) -> Self {
      Self::from_be(Self::from_ne_bytes(bytes))
    }

    #[must_use]
    #[inline]
    pub const fn from_le(x: Self) -> Self {
      x.to_le()
    }

    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; S]) -> Self {
      Self::from_le(Self::from_ne_bytes(bytes))
    }

    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; S]) -> Self {
      Self { bytes }
    }

    #[must_use]
    #[inline]
    pub const fn to_be(self) -> Self {
      #[cfg(target_endian = "big")]
      { self }

      #[cfg(target_endian = "little")]
      { self.swap_bytes() }
    }

    #[must_use]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; S] {
      self.to_be().to_ne_bytes()
    }

    #[must_use]
    #[inline]
    pub const fn to_le(self) -> Self {
      #[cfg(target_endian = "little")]
      { self }

      #[cfg(target_endian = "big")]
      { self.swap_bytes() }
    }

    #[must_use]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; S] {
      self.to_le().to_ne_bytes()
    }

    #[must_use]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; S] {
      self.bytes
    }
  };
  (uint) => {
    $crate::macros::byteorder!(core, uint, true);
  };
  (int) => {
    $crate::macros::byteorder!(core, int, false);
  };
}

pub(crate) use byteorder;
