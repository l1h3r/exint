macro_rules! bin_tools {
  ($name:ident) => {
    #[doc = $crate::utils::include_doc!($name, "reverse_bits")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
      $crate::llapi::swap1::<Self, N>(self)
    }

    #[doc = $crate::utils::include_doc!($name, "swap_bytes")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
      $crate::llapi::swap8::<Self, N>(self)
    }

    #[doc = $crate::utils::include_doc!($name, "rotate_left")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn rotate_left(self, bits: u32) -> Self {
      $crate::llapi::rotl::<Self, N>(self, bits)
    }

    #[doc = $crate::utils::include_doc!($name, "rotate_right")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn rotate_right(self, bits: u32) -> Self {
      $crate::llapi::rotr::<Self, N>(self, bits)
    }

    #[doc(alias = "popcount")]
    #[doc(alias = "popcnt")]
    #[doc = $crate::utils::include_doc!($name, "count_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn count_ones(self) -> u32 {
      $crate::llapi::ctpop::<Self, N>(self)
    }

    #[doc = $crate::utils::include_doc!($name, "count_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn count_zeros(self) -> u32 {
      self.const_not().count_ones()
    }

    #[doc = $crate::utils::include_doc!($name, "leading_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn leading_ones(self) -> u32 {
      self.const_not().leading_zeros()
    }

    #[doc = $crate::utils::include_doc!($name, "leading_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
      $crate::llapi::ctlz::<Self, N>(self)
    }

    #[doc = $crate::utils::include_doc!($name, "trailing_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn trailing_ones(self) -> u32 {
      self.const_not().trailing_zeros()
    }

    #[doc = $crate::utils::include_doc!($name, "trailing_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
      $crate::llapi::cttz::<Self, N>(self)
    }
  };
}

pub(crate) use bin_tools;
