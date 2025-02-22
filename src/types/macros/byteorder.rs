macro_rules! byteorder {
  ($name:ident) => {
    #[doc = $crate::utils::include_doc!($name, "from_be")]
    #[must_use]
    #[inline]
    pub const fn from_be(value: Self) -> Self {
      value.to_be()
    }

    #[doc = $crate::utils::include_doc!($name, "from_le")]
    #[must_use]
    #[inline]
    pub const fn from_le(value: Self) -> Self {
      value.to_le()
    }

    #[doc = $crate::utils::include_doc!($name, "to_be")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn to_be(self) -> Self {
      #[cfg(target_endian = "big")]
      { self }

      #[cfg(target_endian = "little")]
      { self.swap_bytes() }
    }

    #[doc = $crate::utils::include_doc!($name, "to_le")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn to_le(self) -> Self {
      #[cfg(target_endian = "big")]
      { self.swap_bytes() }

      #[cfg(target_endian = "little")]
      { self }
    }

    #[doc = $crate::utils::include_doc!($name, "from_be_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
      Self::from_be(Self::from_ne_bytes(bytes))
    }

    #[doc = $crate::utils::include_doc!($name, "from_le_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
      Self::from_le(Self::from_ne_bytes(bytes))
    }

    #[doc = $crate::utils::include_doc!($name, "from_ne_bytes")]
    #[must_use]
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
      Self { bytes }
    }

    #[doc = $crate::utils::include_doc!($name, "to_be_bytes")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; N] {
      self.to_be().to_ne_bytes()
    }

    #[doc = $crate::utils::include_doc!($name, "to_le_bytes")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
      self.to_le().to_ne_bytes()
    }

    #[doc = $crate::utils::include_doc!($name, "to_ne_bytes")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
      self.bytes
    }
  };
}

pub(crate) use byteorder;
