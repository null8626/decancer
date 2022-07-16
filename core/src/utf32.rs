use std::slice::Iter;
use crate::{utf8, utf16};

#[doc(hidden)]
pub struct IterWrapper<'a, T> {
  inner: Iter<'a, T>,
}

impl<'a, T> IterWrapper<'a, T> {
  const fn new(inner: Iter<'a, T>) -> Self {
    Self { inner }
  }
}

impl Iterator for IterWrapper<'_, u32> {
  type Item = u32;
  
  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    Some(*self.inner.next()?)
  }

  #[inline(always)]
  fn count(self) -> usize
    where
      Self: Sized
  {
    self.inner.len()
  }
}

impl Iterator for IterWrapper<'_, char> {
  type Item = u32;
  
  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    Some(*self.inner.next()? as u32)
  }

  #[inline(always)]
  fn count(self) -> usize
    where
      Self: Sized
  {
    self.inner.len()
  }
}

/// A trait for Rust data-types that can yield UTF-32 codepoints. Invalid UTF-8 or UTF-16 bytes will cause the decancer process to be aborted.
pub trait ToCodepoints<'a> {
  type Out: Iterator<Item = u32> + 'a;

  fn to_codepoints(&'a self) -> Self::Out;
  fn approximate_chars(&'a self) -> usize;
}

macro_rules! impl_codepoints_utf8 {
  ($($t:ty),*) => {
    $(impl<'a> ToCodepoints<'a> for $t
    {
      type Out = utf8::Codepoints<'a>;

      #[inline(always)]
      fn to_codepoints(&'a self) -> Self::Out {
        utf8::Codepoints::new(self.as_ref())
      }

      #[inline(always)]
      fn approximate_chars(&'a self) -> usize {
        (self.len() + 3) / 4
      }
    })*
  }
}

macro_rules! impl_codepoints_utf16 {
  ($($t:ty),*) => {
    $(impl<'a> ToCodepoints<'a> for $t
    {
      type Out = utf16::Codepoints<'a>;

      #[inline(always)]
      fn to_codepoints(&'a self) -> Self::Out {
        utf16::Codepoints::new(self.as_ref())
      }

      #[inline(always)]
      fn approximate_chars(&'a self) -> usize {
        self.len()
      }
    })*
  }
}

macro_rules! impl_codepoints_utf32 {
  ($mt:ty, $($t:ty),*) => {
    $(impl<'a> ToCodepoints<'a> for $t
    {
      type Out = IterWrapper<'a, $mt>;

      #[inline(always)]
      fn to_codepoints(&'a self) -> Self::Out {
        IterWrapper::new(self.iter())
      }

      #[inline(always)]
      fn approximate_chars(&'a self) -> usize {
        self.len()
      }
    })*
  }
}

macro_rules! impl_codepoints_string {
  ($($t:ty),*) => {
    $(impl<'a> ToCodepoints<'a> for $t
    {
      type Out = utf8::Codepoints<'a>;

      #[inline(always)]
      fn to_codepoints(&'a self) -> Self::Out {
        utf8::Codepoints::new(self.as_bytes())
      }

      #[inline(always)]
      fn approximate_chars(&'a self) -> usize {
        (self.len() + 3) / 4
      }
    })*
  }
}

impl_codepoints_utf8! { [u8], Vec<u8> }
impl_codepoints_utf16! { [u16], Vec<u16> }
impl_codepoints_utf32! { u32, [u32], Vec<u32> }
impl_codepoints_utf32! { char, [char], Vec<char> }
impl_codepoints_string! { str, String }