use super::Class;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Level(u8);

pub(crate) const MAX_EXPLICIT_DEPTH: u8 = 125;
pub(crate) const MAX_IMPLICIT_DEPTH: u8 = MAX_EXPLICIT_DEPTH + 1;

impl Level {
  pub(crate) const fn ltr() -> Self {
    Self(0)
  }

  pub(crate) const fn rtl() -> Self {
    Self(1)
  }

  pub(crate) const fn level(&self) -> u8 {
    self.0
  }

  pub(crate) const fn new_explicit(number: u8) -> Option<Self> {
    if number <= MAX_EXPLICIT_DEPTH {
      Some(Self(number))
    } else {
      None
    }
  }

  pub(crate) const fn new_implicit(number: u8) -> Option<Self> {
    if number <= MAX_IMPLICIT_DEPTH {
      Some(Self(number))
    } else {
      None
    }
  }

  #[inline(always)]
  pub fn lower(&mut self, amount: u8) -> Option<()> {
    self.0 = self.0.checked_sub(amount)?;

    Some(())
  }

  #[inline(always)]
  pub fn raise(&mut self, amount: u8) -> Option<()> {
    let number = self.0.checked_add(amount)?;

    if number <= MAX_IMPLICIT_DEPTH {
      self.0 = number;

      Some(())
    } else {
      None
    }
  }

  pub(crate) const fn is_ltr(&self) -> bool {
    self.0 % 2 == 0
  }

  pub(crate) const fn is_rtl(&self) -> bool {
    self.0 % 2 == 1
  }

  pub(crate) const fn class(&self) -> Class {
    if self.is_ltr() {
      Class::L
    } else {
      Class::R
    }
  }

  pub(crate) const fn new_explicit_next_ltr(&self) -> Option<Self> {
    Self::new_explicit((self.0 + 2) & !1)
  }

  pub(crate) const fn new_explicit_next_rtl(&self) -> Option<Self> {
    Self::new_explicit((self.0 + 1) | 1)
  }

  pub(crate) const fn new_lowest_ge_rtl(&self) -> Option<Self> {
    Self::new_implicit(self.0 | 1)
  }
}
