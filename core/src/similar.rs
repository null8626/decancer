#[cfg(feature = "leetspeak")]
use crate::leetspeak;
use crate::{
  codepoints::CODEPOINTS,
  util::{read_u16_le, unwrap_or_ret},
};
use std::{ops::Range, str::Chars};

pub(crate) const SIMILAR_START: u16 = read_u16_le(unsafe { CODEPOINTS.offset(2) });
pub(crate) const SIMILAR_END: u16 = read_u16_le(unsafe { CODEPOINTS.offset(4) });

pub(crate) fn is(self_char: char, other_char: char) -> bool {
  // SAFETY: even if there is no lowercase mapping for some codepoints, it would just return itself.
  // therefore, the first iteration and/or codepoint always exists.
  let self_char = unsafe { self_char.to_lowercase().next().unwrap_unchecked() as u32 };
  let other_char = unsafe { other_char.to_lowercase().next().unwrap_unchecked() as u32 };

  if self_char == other_char {
    return true;
  } else if self_char <= 0x7f && other_char <= 0x7f {
    let mut contains_a = false;
    let mut contains_b = false;

    for offset in SIMILAR_START..SIMILAR_END {
      let cur = unsafe { *(CODEPOINTS.offset(offset as _)) };
      let sim = cur & 0x7f;

      if sim == (self_char as u8) {
        contains_a = true;
      }

      if sim == (other_char as u8) {
        contains_b = true;
      }

      if contains_a && contains_b {
        return true;
      } else if cur >= 0x80 {
        contains_a = false;
        contains_b = false;
      }
    }
  }

  false
}

struct CachedPeek<'a> {
  iterator: Chars<'a>,
  cache: Vec<char>,
  index: usize,
  ended: bool,
}

impl<'a> CachedPeek<'a> {
  #[inline(always)]
  pub(crate) fn new(iterator: Chars<'a>, first: char) -> Self {
    Self {
      iterator,
      cache: vec![first],
      index: 0,
      ended: false,
    }
  }

  fn next_value(&mut self) -> Option<char> {
    self.index += 1;

    match self.cache.get(self.index) {
      Some(value) => Some(*value),

      None => {
        let value = self.iterator.next()?;
        self.cache.push(value);

        Some(value)
      }
    }
  }

  #[inline(always)]
  fn restart(&mut self) {
    self.index = 0;
    self.ended = false;
  }
}

impl<'a> Iterator for CachedPeek<'a> {
  type Item = (char, Option<char>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.ended {
      return None;
    }

    // SAFETY: the current index always points to an existing element
    let current = unsafe { *self.cache.get_unchecked(self.index) };
    let next_element = self.next_value();

    if next_element.is_none() {
      self.ended = true;
    }

    Some((current, next_element))
  }
}

/// A matcher iterator around a string that yields a non-inclusive [`Range`] whenever it detects a similar match.
#[must_use]
pub struct Matcher<'a, 'b> {
  self_iterator: Chars<'a>,
  #[cfg(feature = "leetspeak")]
  self_str: &'a str,
  self_index: usize,
  other_iterator: CachedPeek<'b>,
}

impl<'a, 'b> Matcher<'a, 'b> {
  pub(crate) fn new(mut self_str: &'a str, other_str: &'b str) -> Self {
    let mut other_chars = other_str.chars();
    let other_first = other_chars.next();

    if other_first.is_none() || self_str.len() < other_str.len() {
      self_str = "";
    }

    // SAFETY: self_str = "" would immediately cancel out the first iteration.
    Self {
      self_iterator: self_str.chars(),
      #[cfg(feature = "leetspeak")]
      self_str,
      self_index: 0,
      other_iterator: CachedPeek::new(other_chars, unsafe { other_first.unwrap_unchecked() }),
    }
  }

  #[cfg(feature = "leetspeak")]
  fn matches_leetspeak(&mut self, other_char: char) -> Option<usize> {
    // SAFETY: already guaranteed to be within the string's bounds.
    let haystack = unsafe { self.self_str.get_unchecked(self.self_index..) };
    let matched_len = leetspeak::find(haystack, other_char as _)?;

    // SAFETY: this will never go out of bounds as well
    //         the furthest it would go would be an empty string.
    self.self_iterator =
      unsafe { self.self_str.get_unchecked(self.self_index + matched_len..) }.chars();

    Some(matched_len)
  }

  #[cfg_attr(not(feature = "leetspeak"), inline(always))]
  fn matches_character(&self, self_char: char, other_char: char) -> Option<usize> {
    if is(self_char, other_char) {
      Some(other_char.len_utf8())
    } else {
      None
    }
  }

  fn matches(&mut self, self_char: char, other_char: char) -> Option<usize> {
    #[cfg(feature = "leetspeak")]
    {
      self
        .matches_leetspeak(other_char)
        .or_else(|| self.matches_character(self_char, other_char))
    }

    #[cfg(not(feature = "leetspeak"))]
    {
      self.matches_character(self_char, other_char)
    }
  }

  pub(crate) fn is_equal(self_str: &'a str, other_str: &'b str) -> bool {
    let mut iter = Self::new(self_str, other_str);
    let mat = unwrap_or_ret!(iter.next(), false);

    mat.start == 0 && mat.end == self_str.len()
  }

  fn skip_until(&mut self, other_char: char) -> Option<(usize, usize)> {
    let mut skipped = 0;

    loop {
      let next_self_char = self.self_iterator.next()?;

      if let Some(matched_skip) = self.matches(next_self_char, other_char) {
        return Some((skipped, matched_skip));
      } else {
        skipped += next_self_char.len_utf8();
      }
    }
  }
}

impl<'a, 'b> Iterator for Matcher<'a, 'b> {
  type Item = Range<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    self.other_iterator.restart();

    let mut current_other = self.other_iterator.next()?;

    let (skipped, matched_skip) = self.skip_until(current_other.0)?;

    let mut start_index = self.self_index + skipped;
    self.self_index = start_index + matched_skip;
    let mut last_match_end = start_index;
    let mut current_separator: Option<char> = None;

    while let Some(next_self_char) = self.self_iterator.next() {
      if let Some(next_other) = current_other.1 {
        if let Some(matched_skip) = self.matches(next_self_char, next_other) {
          self.self_index += matched_skip;
          last_match_end = self.self_index;
          current_separator = None;

          current_other = unwrap_or_ret!(
            self.other_iterator.next(),
            Some(start_index..last_match_end)
          );

          continue;
        }
      }

      if let Some(matched_skip) = self.matches(next_self_char, current_other.0) {
        self.self_index += matched_skip;
        last_match_end = self.self_index;
        current_separator = None;
      } else {
        self.self_index += next_self_char.len_utf8();

        match current_separator {
          Some(separator) => {
            if !is(next_self_char, separator) {
              if current_other.1.is_none() {
                return Some(start_index..last_match_end);
              }

              self.other_iterator.restart();

              current_separator = None;
              // SAFETY: this state of the program wouldn't be accessible if the first iteration returns a None
              current_other = unsafe { self.other_iterator.next().unwrap_unchecked() };

              let (skipped, matched_skip) = self.skip_until(current_other.0)?;

              start_index = self.self_index + skipped;
              self.self_index = start_index + matched_skip;
            }
          }

          None => {
            current_separator.replace(next_self_char);
          }
        }
      }
    }

    if current_other.1.is_none() {
      Some(start_index..last_match_end)
    } else {
      None
    }
  }
}
