use crate::codepoints::CODEPOINTS;
#[cfg(feature = "leetspeak")]
use crate::leetspeak;
use std::{char, iter::FusedIterator, ops::Range, str::Chars};

pub(crate) const SIMILAR_START: u16 = CODEPOINTS.u16_at(2);
pub(crate) const SIMILAR_END: u16 = CODEPOINTS.u16_at(4);

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
      Some(&value) => Some(value),

      None => {
        let value = self.iterator.next()?;
        self.cache.push(value);

        Some(value)
      },
    }
  }

  #[inline(always)]
  fn restart(&mut self) {
    self.index = 0;
    self.ended = false;
  }
}

impl Iterator for CachedPeek<'_> {
  type Item = (char, Option<char>);

  fn next(&mut self) -> Option<Self::Item> {
    if self.ended {
      return None;
    }

    let current = self.cache[self.index];
    let next_element = self.next_value();

    if next_element.is_none() {
      self.ended = true;
    }

    Some((current, next_element))
  }
}

/// A matcher iterator around a string that yields a non-inclusive [`Range`] whenever it detects a similar match.
pub struct Matcher<'a, 'b> {
  self_iterator: Chars<'a>,
  #[cfg(feature = "leetspeak")]
  self_str: &'a str,
  self_index: usize,
  start_index: usize,
  other_iterator: CachedPeek<'b>,
}

impl<'a, 'b> Matcher<'a, 'b> {
  pub(crate) fn new(mut self_str: &'a str, other_str: &'b str) -> Self {
    let mut other_chars = other_str.chars();
    let other_first = other_chars.next();

    if other_first.is_none() || self_str.len() < other_str.len() {
      self_str = "";
    }

    Self {
      self_iterator: self_str.chars(),
      #[cfg(feature = "leetspeak")]
      self_str,
      self_index: 0,
      start_index: 0,
      other_iterator: CachedPeek::new(other_chars, other_first.unwrap()),
    }
  }

  #[cfg(feature = "leetspeak")]
  fn matches_leetspeak(&mut self, other_char: char) -> Option<usize> {
    let haystack = &self.self_str[self.self_index..];
    let matched_len = leetspeak::find(haystack.as_bytes(), other_char as _)?;

    self.self_iterator = haystack[matched_len..].chars();

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
    let matched_skip;

    loop {
      let next_self_char = self.self_iterator.next()?;

      if let Some(matched_skip_inner) = self.matches(next_self_char, current_other.0) {
        matched_skip = matched_skip_inner;
        break;
      }

      skipped += next_self_char.len_utf8();
    }

    self.start_index = self.self_index + skipped;
    self.self_index = self.start_index + matched_skip;

    Some(current_other)
  }
}

impl Iterator for Matcher<'_, '_> {
  type Item = Range<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    let mut current_other = self.restart()?;
    let mut last_match_end = self.start_index;

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
          Some(new) => current_other = new,
          None => return Some(self.start_index..last_match_end),
        }

        continue;
      }

      if let Some(matched_skip) = self.matches(next_self_char, current_other.0) {
        self.self_index += matched_skip;
        last_match_end = self.self_index;
        #[cfg(feature = "separators")]
        {
          current_separator = None;
        }
      } else {
        self.self_index += next_self_char.len_utf8();

        #[cfg(feature = "separators")]
        match current_separator {
          Some(separator) => {
            if !is(next_self_char, separator) {
              if current_other.1.is_none() {
                return Some(self.start_index..last_match_end);
              }

              current_separator = None;
              current_other = self.restart()?;
            }
          },

          None => {
            current_separator.replace(next_self_char);
          },
        }

        #[cfg(not(feature = "separators"))]
        {
          if current_other.1.is_none() {
            return Some(start_index..last_match_end);
          }

          current_other = self.restart()?;
        }
      }
    }

    if current_other.1.is_none() {
      Some(self.start_index..last_match_end)
    } else {
      None
    }
  }
}

impl FusedIterator for Matcher<'_, '_> {}
