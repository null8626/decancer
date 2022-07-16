struct Restartable<I> {
  iter: I,
  orig: I,
}

impl<I: Clone> Restartable<I> {
  #[inline(always)]
  fn new(it: I) -> Self {
    Self {
      iter: it.clone(),
      orig: it,
    }
  }

  #[inline(always)]
  fn restart(&mut self) {
    self.iter = self.orig.clone();
  }
}

impl<I: Iterator> Iterator for Restartable<I> {
  type Item = I::Item;

  #[inline(always)]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
  }
}

pub(crate) fn contains<I, B, S, F>(mut big: B, smol: S, cmp_fn: F) -> bool
where
  I: Copy,
  B: Iterator<Item = I>,
  S: Clone + Iterator<Item = I>,
  F: Fn(I, I) -> bool,
{
  let mut smol_iter = Restartable::new(smol);

  loop {
    if let Some(next_smol) = smol_iter.next() {
      if let Some(next_big) = big.next() {
        if !cmp_fn(next_big, next_smol) {
          smol_iter.restart();
        }
      } else {
        return false;
      }
    } else {
      return true;
    }
  }
}
