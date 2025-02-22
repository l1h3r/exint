pub(crate) enum TypeName {
  Sint(usize),
  Uint(usize),
}

impl TypeName {
  pub(crate) const fn bits(&self) -> usize {
    self.bytes() << 3
  }

  pub(crate) const fn bytes(&self) -> usize {
    match self {
      Self::Sint(bytes) | Self::Uint(bytes) => *bytes,
    }
  }

  pub(crate) const fn uint(&self) -> bool {
    matches!(self, Self::Uint(_))
  }

  pub(crate) const fn next_type(&self) -> Self {
    match self {
      Self::Sint(size) => Self::Sint(*size + 1),
      Self::Uint(size) => Self::Uint(*size + 1),
    }
  }

  pub(crate) fn rust_type(&self) -> String {
    match self {
      Self::Sint(bytes) => format!("int<{}>",  bytes),
      Self::Uint(bytes) => format!("uint<{}>", bytes),
    }
  }

  pub(crate) fn llvm_type(&self) -> String {
    format!("i{}", self.bits())
  }

  pub(crate) fn function(&self, base: &'static str) -> String {
    match self {
      Self::Sint(bytes) => format!("{base}_int_{}", bytes),
      Self::Uint(bytes) => format!("{base}_uint_{}", bytes),
    }
  }

  pub(crate) const fn max_value(&self) -> u128 {
    match self {
      Self::Sint(_) => (1_u128 << (self.bits() - 1)) - 1,
      Self::Uint(16) => u128::MAX,
      Self::Uint(_) => (1_u128 << self.bits()) - 1,
    }
  }

  pub(crate) const fn min_value(&self) -> i128 {
    match self {
      Self::Sint(16) => i128::MIN,
      Self::Sint(_) => -1_i128 << (self.bits() - 1),
      Self::Uint(_) => 0,
    }
  }
}
