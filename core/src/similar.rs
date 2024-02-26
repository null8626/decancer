use crate::{
  codepoints::CODEPOINTS,
  util::{read_u16_le, Restartable, RestartableOpt},
};

pub(crate) const SIMILAR_START: u16 = read_u16_le(unsafe { CODEPOINTS.offset(2) });
pub(crate) const SIMILAR_END: u16 = read_u16_le(unsafe { CODEPOINTS.offset(4) });

pub(crate) fn is(self_char: u32, other_char: char) -> bool {
  // SAFETY: even if there is no lowercase mapping for some codepoints, it would just return itself.
  // therefore, the first iteration and/or codepoint always exists.
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

#[derive(Copy, Clone)]
struct PeekResult {
  current: char,
  next: Option<char>,
}

impl PeekResult {
  const fn new(current: char, next: Option<char>) -> Self {
    Self { current, next }
  }

  #[inline(always)]
  fn matches_current(&mut self, self_char: char) -> bool {
    is(self_char as _, self.current)
  }

  fn matches_next(&self, self_char: char) -> bool {
    match self.next {
      Some(next) => is(self_char as _, next),
      None => false,
    }
  }
}

struct Peek<I> {
  iterator: I,
  current: char,
  ended: bool,
}

impl<I> Peek<I>
where
  I: Iterator<Item = char>,
{
  #[inline(always)]
  fn new(mut iterator: I) -> Option<Self> {
    iterator.next().map(|current| Self {
      iterator,
      current,
      ended: false,
    })
  }
}

impl<I> Iterator for Peek<I>
where
  I: Iterator<Item = char>,
{
  type Item = PeekResult;

  fn next(&mut self) -> Option<Self::Item> {
    if self.ended {
      return None;
    }

    let current = self.current;
    let next_char = self.iterator.next();

    match next_char {
      Some(next_char_inner) => self.current = next_char_inner,
      None => self.ended = true,
    };

    Some(PeekResult::new(current, next_char))
  }
}

impl<I> RestartableOpt<PeekResult> for Peek<I>
where
  I: Iterator<Item = char>,
{
  #[inline(always)]
  fn restart_callback(&mut self) {
    self.ended = false;
  }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum State {
  JustStarted = 0,
  Matched,
  Separation,
}

fn truly_ended<I>(matched: bool, state: State, other_iterator: &mut I) -> bool
where
  I: Iterator<Item = PeekResult>,
{
  matched
    || (state == State::Matched
      && match other_iterator.next() {
        Some(last) => last.next.is_none(),
        None => true,
      })
}

pub(crate) fn is_iter<I>(mut self_iterator: I, other_iterator: I, is_equal: bool) -> bool
where
  I: Iterator<Item = char>,
{
  let mut other_iterator = match Peek::new(other_iterator) {
    Some(iterator) => iterator,
    None => return self_iterator.next().is_none(),
  };

  // SAFETY: this is impossible to be None.
  let mut current_other = unsafe { other_iterator.next().unwrap_unchecked() };
  let mut current_separator = None;
  let mut matched = false;
  let mut state = State::JustStarted;

  for self_char in self_iterator {
    if current_other.matches_next(self_char) && state != State::JustStarted {
      state = State::Matched;
      current_separator = None;
      matched = other_iterator.ended;

      current_other = match other_iterator.next() {
        Some(current) => current,
        None => return true,
      };
    } else if current_other.matches_current(self_char) {
      state = State::Matched;
      matched = other_iterator.ended;
      current_separator = None;

      continue;
    } else {
      if is_equal && matched {
        return false;
      }

      match current_separator {
        Some(separator) => {
          if !is(self_char as _, separator) {
            break;
          }
        }

        None => {
          if state == State::JustStarted {
            return false;
          } else if is_equal {
            state = State::Separation;
          }

          current_separator.replace(self_char);
        }
      }
    }
  }

  truly_ended(matched, state, &mut other_iterator)
}

pub(crate) fn is_contains<I>(mut self_iterator: I, other_iterator: I) -> bool
where
  I: Iterator<Item = char>,
{
  let mut other_iterator = match Peek::new(other_iterator) {
    Some(iterator) => Restartable::new(iterator),
    None => return self_iterator.next().is_none(),
  };

  let mut self_char_skip = unsafe { self_iterator.next().unwrap_unchecked() as _ };
  let mut current_other;

  loop {
    if is(self_char_skip as _, other_iterator.current) {
      current_other = match other_iterator.next() {
        Some(current_other) => current_other,
        None => return false,
      };

      break;
    }

    self_char_skip = match self_iterator.next() {
      Some(output) => output,
      None => return false,
    };
  }

  let mut current_separator = None;
  let mut matched = false;
  let mut state = State::Matched;

  for self_char in self_iterator {
    if current_other.matches_next(self_char) && state != State::JustStarted {
      state = State::Matched;
      current_separator = None;
      matched = other_iterator.ended;

      current_other = match other_iterator.next() {
        Some(current) => current,
        None => return true,
      };
    } else if current_other.matches_current(self_char) {
      state = State::Matched;
      matched = other_iterator.ended;
      current_separator = None;

      continue;
    } else {
      match current_separator {
        Some(separator) => {
          if truly_ended(matched, state, &mut other_iterator) {
            return true;
          }

          if !is(self_char as _, separator) {
            other_iterator.restart();

            // SAFETY: this is impossible to be None
            current_separator = None;
            current_other = unsafe { other_iterator.next().unwrap_unchecked() };
            state = State::JustStarted;
          }
        }

        None => {
          current_separator.replace(self_char);
        }
      }
    }
  }

  truly_ended(matched, state, &mut other_iterator)
}

#[inline(always)]
pub(crate) fn is_str<'a>(s: &'a str, o: &'a str, is_equal: bool) -> bool {
  is_iter(s.chars(), o.chars(), is_equal)
}
