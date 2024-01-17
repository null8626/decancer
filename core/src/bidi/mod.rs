mod brackets;
mod class;
mod level;
mod paragraph;

use crate::util::read_u16_le;
pub(crate) use brackets::{BracketPair, OpeningBracket};
pub(crate) use class::Class;
pub(crate) use level::Level;
pub(crate) use paragraph::{IsolatingRunSequence, OverrideStatus, Paragraph};

const BIDI: *const u8 = include_bytes!("../../bin/bidi.bin").as_ptr();

const BIDI_DICTIONARY_OFFSET: u16 = read_u16_le(BIDI);
const BIDI_DICTIONARY_COUNT: u16 = unsafe { read_u16_le(BIDI.offset(2)) };
const BIDI_BRACKETS_COUNT: u16 = ((BIDI_DICTIONARY_OFFSET - 4) / 5) - 1;
