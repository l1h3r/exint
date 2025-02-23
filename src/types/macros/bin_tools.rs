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

    #[doc = $crate::utils::include_doc!($name, "isolate_most_significant_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn isolate_most_significant_one(self) -> Self {
      Self::ONE
        .const_shl(Self::BITS - 1)
        .wrapping_shr(self.leading_zeros())
        .const_band(self)
    }

    #[doc = $crate::utils::include_doc!($name, "isolate_least_significant_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn isolate_least_significant_one(self) -> Self {
      self.const_band(self.wrapping_neg())
    }
  };
  ($outer:ident<$inner:ident>) => {
    #[doc = $crate::utils::include_doc!($outer, $inner, "reverse_bits")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn reverse_bits(self) -> Self {
      Self(self.0.reverse_bits())
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "swap_bytes")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn swap_bytes(self) -> Self {
      Self(self.0.swap_bytes())
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "rotate_left")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn rotate_left(self, bits: u32) -> Self {
      Self(self.0.rotate_left(bits))
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "rotate_right")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn rotate_right(self, bits: u32) -> Self {
      Self(self.0.rotate_right(bits))
    }

    #[doc(alias = "popcount")]
    #[doc(alias = "popcnt")]
    #[doc = $crate::utils::include_doc!($outer, $inner, "count_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn count_ones(self) -> u32 {
      self.0.count_ones()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "count_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn count_zeros(self) -> u32 {
      self.0.count_zeros()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "leading_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn leading_ones(self) -> u32 {
      self.0.leading_ones()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "leading_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn leading_zeros(self) -> u32 {
      self.0.leading_zeros()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "trailing_ones")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn trailing_ones(self) -> u32 {
      self.0.trailing_ones()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "trailing_zeros")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn trailing_zeros(self) -> u32 {
      self.0.trailing_zeros()
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "isolate_most_significant_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn isolate_most_significant_one(self) -> Self {
      Self(self.0.isolate_most_significant_one())
    }

    #[doc = $crate::utils::include_doc!($outer, $inner, "isolate_least_significant_one")]
    #[cfg(feature = "isolate_most_least_significant_one")]
    #[must_use = $crate::utils::must_use_doc!()]
    #[inline]
    pub const fn isolate_least_significant_one(self) -> Self {
      Self(self.0.isolate_least_significant_one())
    }
  };
}

pub(crate) use bin_tools;
