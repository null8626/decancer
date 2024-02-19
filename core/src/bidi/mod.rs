mod class;

use crate::util::read_u16_le;
pub(crate) use class::Class;

const BIDI: *const u8 = include_bytes!("../../bin/bidi.bin").as_ptr();

const BIDI_DICTIONARY_OFFSET: u16 = read_u16_le(BIDI);
const BIDI_DICTIONARY_COUNT: u16 = unsafe { read_u16_le(BIDI.offset(2)) };
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
