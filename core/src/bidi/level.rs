// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use super::{super::Error, Class};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Level(pub(in super::super) u8);

pub const MAX_EXPLICIT_DEPTH: u8 = 125;
pub const MAX_IMPLICIT_DEPTH: u8 = MAX_EXPLICIT_DEPTH + 1;

impl Level {
  pub(in super::super) const LTR: Self = Self(0);
  pub(in super::super) const RTL: Self = Self(1);

  pub(in super::super) const fn new_explicit(number: u8) -> Result<Self, Error> {
    if number <= MAX_EXPLICIT_DEPTH {
      Ok(Self(number))
    } else {
      Err(Error::LevelExplicitOverflow)
    }
  }

  pub(in super::super) const fn new_implicit(number: u8) -> Result<Self, Error> {
    if number <= MAX_IMPLICIT_DEPTH {
      Ok(Self(number))
    } else {
      Err(Error::LevelImplicitOverflow)
    }
  }

  pub(in super::super) const fn lower(&mut self, amount: u8) -> Result<(), Error> {
    match self.0.checked_sub(amount) {
      Some(result) => {
        self.0 = result;

        Ok(())
      },

      None => Err(Error::LevelModificationUnderflow),
    }
  }

  pub(in super::super) const fn raise(&mut self, amount: u8) -> Result<(), Error> {
    if let Some(number) = self.0.checked_add(amount)
      && number <= MAX_IMPLICIT_DEPTH
    {
      self.0 = number;

      return Ok(());
    }

    Err(Error::LevelModificationOverflow)
  }

  pub(in super::super) const fn is_rtl(self) -> bool {
    self.0 % 2 == 1
  }

  pub(in super::super) const fn class(self) -> Class {
    if self.is_rtl() { Class::R } else { Class::L }
  }

  pub(in super::super) const fn new_explicit_next_ltr(self) -> Result<Self, Error> {
    Self::new_explicit((self.0 + 2) & !1)
  }

  pub(in super::super) const fn new_explicit_next_rtl(self) -> Result<Self, Error> {
    Self::new_explicit((self.0 + 1) | 1)
  }

  pub(in super::super) const fn new_lowest_ge_rtl(self) -> Result<Self, Error> {
    Self::new_implicit(self.0 | 1)
  }
}
