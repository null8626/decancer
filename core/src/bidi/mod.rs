// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

mod class;

use crate::util::Binary;
pub(crate) use class::Class;

const BIDI: Binary = Binary::new(include_bytes!("../../bin/bidi.bin"));

const BIDI_DICTIONARY_OFFSET: u16 = BIDI.u16_at(0);
const BIDI_DICTIONARY_COUNT: u16 = BIDI.u16_at(2);
const BIDI_BRACKETS_COUNT: u16 = ((BIDI_DICTIONARY_OFFSET - 4) / 5) - 1;

mod brackets;
mod level;
mod paragraph;

use brackets::{BracketPair, OpeningBracket};
use paragraph::OverrideStatus;

pub(crate) use level::Level;
#[cfg(test)]
pub(crate) use paragraph::IsolatingRunSequence;
pub(crate) use paragraph::Paragraph;
