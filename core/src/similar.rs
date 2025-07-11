#[cfg(feature = "leetspeak")]
use crate::leetspeak;
use crate::{
  codepoints::CODEPOINTS,
  util::{Cached, CachedPeek},
};
use std::{char, iter::FusedIterator, ops::Range};

pub(crate) const SIMILAR_START: u16 = CODEPOINTS.u16_at(2);
pub(crate) const SIMILAR_END: u16 = CODEPOINTS.u16_at(4);

#[cfg(feature = "separators")]
fn is_exact(self_char: char, other_char: char) -> bool {
  let self_char = self_char.to_lowercase().next().unwrap_or(self_char) as u32;
  let other_char = other_char.to_lowercase().next().unwrap_or(other_char) as u32;

  self_char == other_char
}

pub(crate) fn is(self_char: char, other_char: char) -> bool {
  let self_char = self_char.to_lowercase().next().unwrap_or(self_char) as u32;
  let other_char = other_char.to_lowercase().next().unwrap_or(other_char) as u32;

  if self_char == other_char {
    return true;
  } else if self_char <= 0x7f && other_char <= 0x7f {
    let mut id = 0;

    for offset in SIMILAR_START..SIMILAR_END {
      let cur = CODEPOINTS.at(offset as _);
      let sim = cur & 0x7f;

      if sim == (self_char as u8) {
        id |= 1;
      } else if sim == (other_char as u8) {
        id |= 2;
      }

      if id == 3 {
        return true;
      } else if cur >= 0x80 {
        id = 0;
      }
    }
  }

  false
}

struct ExplicitStartingPosition {
  index: usize,
  start_index: usize,
  size: usize,
}

/// A matcher iterator around a string that yields a non-inclusive [`Range`] whenever it detects a similar match.
pub struct Matcher<'a, 'b> {
  self_iterator: Cached<'a>,
  #[cfg(feature = "leetspeak")]
  self_str: &'a str,
  explicit_starting_position: Option<ExplicitStartingPosition>,
  self_index: usize,
  start_index: usize,
  other_iterator: CachedPeek<'b>,
}

impl<'a, 'b> Matcher<'a, 'b> {
  pub(crate) fn new(mut self_str: &'a str, other_str: &'b str) -> Self {
    if other_str.is_empty() || self_str.len() < other_str.len() {
      self_str = "";
    }

    Self {
      self_iterator: Cached::new(self_str.chars()),
      #[cfg(feature = "leetspeak")]
      self_str,
      explicit_starting_position: None,
      self_index: 0,
      start_index: 0,
      other_iterator: CachedPeek::new(other_str.chars()),
    }
  }

  #[cfg(feature = "leetspeak")]
  fn matches_leetspeak(&mut self, other_char: char) -> Option<usize> {
    let haystack = &self.self_str[self.self_index..];
    let matched_len = leetspeak::find(haystack.as_bytes(), other_char as _)?;

    self.self_iterator = Cached::new(haystack[matched_len..].chars());

    Some(matched_len)
  }

  #[cfg_attr(not(feature = "leetspeak"), inline(always))]
  fn matches_character(self_char: char, other_char: char) -> Option<usize> {
    if is(self_char, other_char) {
      Some(other_char.len_utf8())
    } else {
      None
    }
  }

  fn matches(&mut self, self_char: char, other_char: char) -> Option<usize> {
    #[cfg(feature = "leetspeak")]
    {
      Self::matches_character(self_char, other_char).or_else(|| self.matches_leetspeak(other_char))
    }

    #[cfg(not(feature = "leetspeak"))]
    {
      Self::matches_character(self_char, other_char)
    }
  }

  pub(crate) fn is_equal(self_str: &'a str, other_str: &'b str) -> bool {
    let mut iter = Self::new(self_str, other_str);
    let Some(mat) = iter.next() else {
      return false;
    };

    mat.start == 0 && mat.end == self_str.len()
  }

  fn restart(&mut self) -> Option<(char, Option<char>)> {
    self.other_iterator.restart();

    let current_other = self.other_iterator.next()?;
    let mut skipped = 0;

    if let Some(explicit_starting_position) = self.explicit_starting_position.take() {
      self
        .self_iterator
        .set_index(explicit_starting_position.index);
      self.start_index = explicit_starting_position.start_index;
      self.self_index = self.start_index + explicit_starting_position.size;

      return Some(current_other);
    }

    loop {
      let next_self_char = self.self_iterator.next()?;

      if let Some(matched_skip) = self.matches(next_self_char, current_other.0) {
        self.start_index = self.self_index + skipped;
        self.self_index = self.start_index + matched_skip;

        return Some(current_other);
      }

      skipped += next_self_char.len_utf8();
    }
  }
}

impl Iterator for Matcher<'_, '_> {
  type Item = Range<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut current_other = self.restart()?;
    let mut last_match_end = self.self_index;
    let first_other = current_other.0;
    let mut completed = current_other.1.is_none();

    #[cfg(feature = "separators")]
    let mut current_separator: Option<char> = None;

    while let Some(next_self_char) = self.self_iterator.next() {
      if let Some(matched_skip) = current_other
        .1
        .and_then(|next_other| self.matches(next_self_char, next_other))
      {
        self.self_index += matched_skip;
        last_match_end = self.self_index;

        #[cfg(feature = "separators")]
        {
          current_separator = None;
        }

        match self.other_iterator.next() {
          Some(new) => {
            current_other = new;

            if current_other.1.is_none() {
              completed = true;
            }
          },

          None => return Some(self.start_index..last_match_end),
        }

        continue;
      }

      if let Some(matched_skip) = self.matches(next_self_char, current_other.0) {
        self.self_index += matched_skip;
        last_match_end = self.self_index;

        if current_other.1.is_none() {
          completed = true;
        }

        #[cfg(feature = "separators")]
        {
          current_separator = None;
        }
      } else {
        if let Some(matched_skip) = self.matches(next_self_char, first_other) {
          self
            .explicit_starting_position
            .replace(ExplicitStartingPosition {
              index: self.self_iterator.index(),
              start_index: self.self_index,
              size: matched_skip,
            });

          if completed {
            return Some(self.start_index..last_match_end);
          }

          current_other = self.restart()?;
          continue;
        }

        self.self_index += next_self_char.len_utf8();

        #[cfg(feature = "separators")]
        match current_separator {
          Some(separator) => {
            if !is_exact(next_self_char, separator) {
              if completed {
                return Some(self.start_index..last_match_end);
              }

              current_separator = None;
              current_other = self.restart()?;
            }
          },

          None => {
            if next_self_char.is_ascii_alphabetic() {
              if completed {
                return Some(self.start_index..last_match_end);
              }

              current_other = self.restart()?;
            } else {
              current_separator.replace(next_self_char);
            }
          },
        }

        #[cfg(not(feature = "separators"))]
        {
          if completed {
            return Some(self.start_index..last_match_end);
          }

          current_other = self.restart()?;
        }
      }
    }

    if completed {
      Some(self.start_index..last_match_end)
    } else {
      None
    }
  }
}

impl FusedIterator for Matcher<'_, '_> {}
