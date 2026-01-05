// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::Class;
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
    match self.0.checked_sub(amount) {
      Some(result) => {
        self.0 = result;

        Ok(())
      },

      None => Err(Error::LevelModificationUnderflow),
    }
  }

  pub(crate) fn raise(&mut self, amount: u8) -> Result<(), Error> {
    if let Some(number) = self.0.checked_add(amount) {
      if number <= MAX_IMPLICIT_DEPTH {
        self.0 = number;

        return Ok(());
      }
    }

    Err(Error::LevelModificationOverflow)
  }

  pub(crate) const fn is_rtl(self) -> bool {
    self.0 % 2 == 1
  }

  pub(crate) const fn class(self) -> Class {
    if self.is_rtl() {
      Class::R
    } else {
      Class::L
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
