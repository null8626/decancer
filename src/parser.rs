pub(crate) struct Parser<'a> {
  input: &'a [u16],
  output_bytes: Vec<u16>,
  index: usize,
}

impl<'a> Parser<'a> {
  #[inline]
  pub(crate) fn new(slice: &'a [u16]) -> Self {
    Self {
      input: slice,
      output_bytes: Vec::with_capacity(slice.len()),
      index: 0,
    }
  }
  
  #[inline(always)]
  pub(crate) fn push_byte(&mut self, byte: u16) {
    self.output_bytes.push(byte)
  }
  
  pub(crate) fn get(&self) -> u16 {
    let out = self.input.get(self.index);
    
    if !out.is_none() {
      unsafe { *out.unwrap_unchecked() }
    } else {
      0xFFFD
    }
  }
  
  pub(crate) fn get_multiple(&self, amount: usize) -> Vec<u16> {
    let mut result: Vec<u16> = Vec::with_capacity(amount);

    for i in 0..amount {
      let num = self.input.get(self.index + i);
      
      if num.is_none() {
        result.push(0xFFFD);
      } else {
        result.push(unsafe { *num.unwrap_unchecked() });
      }
    }
    
    result
  }
  
  #[inline(always)]
  pub(crate) fn end(&self) -> bool {
    self.index >= self.input.len()
  }
  
  #[inline(always)]
  pub(crate) fn advance(&mut self, offset: usize) {
    self.index += offset;
  }
  
  #[inline(always)]
  pub(crate) fn output(&self) -> String {
    String::from_utf16_lossy(&self.output_bytes[..])
  }
}