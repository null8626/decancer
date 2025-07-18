use std::{ops::Range, str::Chars};

pub(crate) const CODEPOINT_MASK: u32 = 0x000f_ffff;

pub(crate) const fn is_none(code: u32) -> bool {
  matches!(code, 0..=9 | 14..=31 | 127 | 0xd800..=0xf8ff | 0xe01f0..)
}

#[cfg(feature = "options")]
pub(crate) const fn is_special_rtl(code: u32) -> bool {
  matches!(code, 0x200e..=0x200f | 0x202a..=0x202e | 0x2066..=0x2069)
}

#[cfg(feature = "options")]
pub(crate) const fn is_alphanumeric(code: u32) -> bool {
  matches!(code, 48..=57 | 97..=122 | 65..=90 | 32)
}

#[derive(Copy, Clone)]
pub(crate) struct Binary {
  bytes: &'static [u8],
}

impl Binary {
  pub(crate) const fn new(bytes: &'static [u8]) -> Self {
    Self { bytes }
  }

  pub(crate) const fn at(self, offset: usize) -> u8 {
    self.bytes[offset]
  }

  #[inline(always)]
  pub(crate) fn sliced(self, offset: usize, size: usize) -> &'static [u8] {
    &self.bytes[offset..offset + size]
  }

  pub(crate) const fn u16_at(self, offset: usize) -> u16 {
    u16::from_le_bytes([self.at(offset), self.at(offset + 1)])
  }

  pub(crate) const fn u32_at(self, offset: usize) -> u32 {
    u32::from_le_bytes([
      self.at(offset),
      self.at(offset + 1),
      self.at(offset + 2),
      self.at(offset + 3),
    ])
  }
}

// special thanks to https://medium.com/@michealkeines/merge-overlapping-intervals-rust-117a7099f348
// except i've improved upon it :)
pub(crate) fn merge_ranges<T>(ranges: &mut Vec<Range<T>>)
where
  T: Ord + Copy,
{
  if ranges.is_empty() {
    return;
  }

  ranges.sort_by(|a, b| a.start.cmp(&b.start));

  let mut j = 0;

  for i in 1..ranges.len() {
    let current = ranges[i].clone();
    let previous = &mut ranges[j];

    if current.start >= previous.start && current.start <= previous.end {
      previous.end = previous.end.max(current.end);
    } else {
      j += 1;
      ranges[j] = current;
    }
  }

  ranges.truncate(j + 1);
}

macro_rules! error_enum {
  (
    $(#[$enum_attrs:meta])*
    pub enum $enum_name:ident {
      $(
        #[doc = $prop_doc:literal]
        $prop_name:ident,
      )*
    }
  ) => {
    $(#[$enum_attrs])*
    pub enum $enum_name {
      $(
        #[doc = $prop_doc]
        $prop_name,
      )*
    }

    impl std::convert::AsRef<str> for $enum_name {
      fn as_ref(&self) -> &str {
        match self {
          $(
            Self::$prop_name => stringify!($prop_doc),
          )*
        }
      }
    }

    impl std::fmt::Display for $enum_name {
      #[inline(always)]
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <$enum_name as std::convert::AsRef<str>>::as_ref(self))
      }
    }

    impl std::error::Error for $enum_name {}
  }
}

pub(crate) use error_enum;

macro_rules! numbered_enum {
  (
    $(#[$enum_meta:meta])*
    $enum_vis:vis enum $enum_name:ident: $enum_type:ty {
      $($enum_prop:ident = $enum_prop_value:literal,)*
    }
  ) => {
    $(#[$enum_meta])*
    #[repr($enum_type)]
    $enum_vis enum $enum_name {
      $($enum_prop = $enum_prop_value,)*
    }

    impl $enum_name {
      const fn from_number(value: $enum_type) -> Self {
        match value {
          $($enum_prop_value => Self::$enum_prop,)*
          _ => unreachable!(),
        }
      }
    }
  }
}

pub(crate) use numbered_enum;

pub(crate) struct Cached<'a> {
  iterator: Chars<'a>,
  pub(crate) cache: Vec<char>,
  index: usize,
}

impl<'a> Cached<'a> {
  #[inline(always)]
  pub(crate) fn new(iterator: Chars<'a>) -> Self {
    Self {
      iterator,
      cache: Vec::new(),
      index: 0,
    }
  }

  #[inline(always)]
  pub(crate) fn set_index(&mut self, index: usize) {
    self.index = index;
  }

  pub(crate) const fn index(&self) -> usize {
    self.index
  }

  fn get(&mut self, index: usize) -> Option<char> {
    self.cache.get(index).copied().or_else(|| {
      self.iterator.next().map(|value| {
        self.cache.push(value);
        value
      })
    })
  }
}

impl Iterator for Cached<'_> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.get(self.index)?;

    self.index += 1;

    Some(current)
  }
}

pub(crate) struct CachedPeek<'a> {
  iterator: Chars<'a>,
  cache: Vec<char>,
  index: usize,
}

impl<'a> CachedPeek<'a> {
  #[inline(always)]
  pub(crate) fn new(iterator: Chars<'a>) -> Self {
    Self {
      iterator,
      cache: Vec::new(),
      index: 0,
    }
  }

  #[inline(always)]
  pub(crate) fn restart(&mut self) {
    self.index = 0;
  }

  fn get(&mut self, index: usize) -> Option<char> {
    self.cache.get(index).copied().or_else(|| {
      self.iterator.next().map(|value| {
        self.cache.push(value);
        value
      })
    })
  }
}

impl Iterator for CachedPeek<'_> {
  type Item = (char, Option<char>);

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.get(self.index)?;

    self.index += 1;

    Some((current, self.get(self.index)))
  }
}
