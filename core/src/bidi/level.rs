use super::class::{self, Class};
use crate::Error;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Level(pub(crate) u8);

pub(crate) const MAX_EXPLICIT_DEPTH: u8 = 125;
pub(crate) const MAX_IMPLICIT_DEPTH: u8 = MAX_EXPLICIT_DEPTH + 1;

impl Level {
  pub(crate) const fn ltr() -> Self {
    Self(0)
  }

  pub(crate) const fn rtl() -> Self {
    Self(1)
  }

  pub(crate) const fn new_explicit(number: u8) -> Result<Self, Error> {
    if number <= MAX_EXPLICIT_DEPTH {
      Ok(Self(number))
    } else {
      Err(Error::LevelExplicitOverflow)
    }
  }

  pub(crate) const fn new_implicit(number: u8) -> Result<Self, Error> {
    if number <= MAX_IMPLICIT_DEPTH {
      Ok(Self(number))
    } else {
      Err(Error::LevelImplicitOverflow)
    }
  }

  pub(crate) fn lower(&mut self, amount: u8) -> Result<(), Error> {
    self.0 = self
      .0
      .checked_sub(amount)
      .ok_or(Error::LevelModificationUnderflow)?;

    Ok(())
  }

  pub(crate) fn raise(&mut self, amount: u8) -> Result<(), Error> {
    let number = self
      .0
      .checked_add(amount)
      .ok_or(Error::LevelModificationOverflow)?;

    if number <= MAX_IMPLICIT_DEPTH {
      self.0 = number;

      Ok(())
    } else {
      Err(Error::LevelModificationOverflow)
    }
  }

  pub(crate) const fn is_ltr(self) -> bool {
    self.0 % 2 == 0
  }

  pub(crate) const fn is_rtl(self) -> bool {
    self.0 % 2 == 1
  }

  pub(crate) const fn class(self) -> Class {
    if self.is_ltr() {
      class::L
    } else {
      class::R
    }
  }

  pub(crate) const fn new_explicit_next_ltr(self) -> Result<Self, Error> {
    Self::new_explicit((self.0 + 2) & !1)
  }

  pub(crate) const fn new_explicit_next_rtl(self) -> Result<Self, Error> {
    Self::new_explicit((self.0 + 1) | 1)
  }

  pub(crate) const fn new_lowest_ge_rtl(self) -> Result<Self, Error> {
    Self::new_implicit(self.0 | 1)
  }
}
