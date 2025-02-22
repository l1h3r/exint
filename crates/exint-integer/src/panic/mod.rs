#[cold]
#[track_caller]
pub(crate) const fn strict_add() -> ! {
  ::core::panic!("attempt to add with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_sub() -> ! {
  ::core::panic!("attempt to subtract with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_mul() -> ! {
  ::core::panic!("attempt to multiply with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_div() -> ! {
  ::core::panic!("attempt to divide by zero")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_rem() -> ! {
  ::core::panic!("attempt to calculate the remainder with a divisor of zero")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_shl() -> ! {
  ::core::panic!("attempt to shift left with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_shr() -> ! {
  ::core::panic!("attempt to shift right with overflow")
}

#[cold]
#[track_caller]
pub(crate) const fn strict_neg() -> ! {
  ::core::panic!("attempt to negate with overflow")
}
