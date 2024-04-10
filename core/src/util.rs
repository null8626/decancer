use std::{
  cmp::max,
  ops::{Index, IndexMut, Range},
};

pub(crate) const CODEPOINT_MASK: u32 = 0x000f_ffff;

#[derive(Copy, Clone)]
pub(crate) struct Binary<'a> {
  bytes: &'a [u8],
}

impl<'a> Binary<'a> {
  pub(crate) const fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  pub(crate) const fn at(self, offset: usize) -> u8 {
    self.bytes[offset]
  }

  #[inline(always)]
  pub(crate) fn sliced(self, offset: usize, size: usize) -> &'a [u8] {
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

#[inline(always)]
pub(crate) fn sliced<T: Index<Range<usize>> + ?Sized>(
  slicable: &T,
  range: Range<usize>,
) -> &<T as Index<Range<usize>>>::Output {
  slicable.index(range)
}

#[inline(always)]
pub(crate) fn sliced_mut<T: IndexMut<Range<usize>> + ?Sized>(
  slicable: &mut T,
  range: Range<usize>,
) -> &mut <T as Index<Range<usize>>>::Output {
  slicable.index_mut(range)
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
      previous.end = max(current.end, previous.end);
    } else {
      j += 1;
      ranges[j] = current;
    }
  }

  ranges.truncate(j + 1);
}

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

    impl From<$enum_type> for $enum_name {
      fn from(value: $enum_type) -> Self {
        match value {
          $($enum_prop_value => Self::$enum_prop,)*
          _ => panic!(concat!("invalid ", stringify!($enum_name), " value: {}"), value),
        }
      }
    }
  }
}

pub(crate) use numbered_enum;

macro_rules! unwrap_or_ret {
  ($option:expr,$fallback:expr) => {
    match $option {
      Some(inner) => inner,
      None => return $fallback,
    }
  };
}

pub(crate) use unwrap_or_ret;
