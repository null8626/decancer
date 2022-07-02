mod confusables;
mod encoding;

use encoding::*;

fn similar(a: u16, b: u16) -> bool {
  a == b || ((a <= 0xFF) && (b <= 0xFF) && confusables::similar().any(|x| {
    x.contains(a as _) && x.contains(b as _)
  }))
}

pub fn contains(a: &[u16], b: &[u16]) -> bool {
  if a.len() == 0 || a.len() < b.len() {
    return false;
  }

  let mut j = 0usize;
  for (&x, &y) in a.iter().zip(b) {
    if similar(x, y) {
      j += 1;

      if j == b.len() {
        return true;
      }
    } else {
      j = 0;
    }
  }

  false
}

pub fn decancer(input: &[u16]) -> String {
  let mut output = String::with_capacity(input.len());

  // for_each so we can have return (implement some sort of goto in rust)
  Codepoints::from(input)
    .filter(|&x| ((x > 31 && x < 127) || (x > 159 && x < 0x300) || x > 0x36F) && x != 0x20E3 && x != 0xFE0F && x != 0x489)
    .for_each(|x| {
      for num in confusables::numerical() {
        if x >= num && x <= (num + 9) {
          return output.push(unsafe { char::from_u32_unchecked(x - num + 0x30) });
        }
      }
  
      for (key, value) in confusables::misc_case_sensitive() {
        if value.contains(x) {
          for k in key {
            output.push(k as char);
          }
  
          return;
        }
      }

      if let Some(c22) = char::from_u32(x) {
        c22.to_lowercase().for_each(|c2| {
          let c = c2 as u32;

          for pat in confusables::alphabetical_pattern() {
            if c >= pat && c <= (pat + 25) {
              return output.push(unsafe { char::from_u32_unchecked(c - pat + 0x61) });
            }
          }

          for (i, arr) in confusables::alphabetical().enumerate() {
            if arr.contains(c) {
              return output.push(unsafe { char::from_u32_unchecked((i as u32) + 0x61) });
            }
          }

          for (a, b) in confusables::misc() {
            if b.contains(c) {
              return output.push(a as char);
            }
          }

          if let Some(t) = char::from_u32(c) {
            output.push(t);
          }
        });
      }
    });

  output.retain(|c2| {
    let (a, b) = charcodes(c2 as _);

    if a != 0xFFFD && (a < 0xD800 || a > 0xDB7F) && a < 0xFFF0 {
      if let Some(b2) = b {
        b2 != 0xFFFD && (b2 < 0xD800 || b2 > 0xDB7F) && b2 < 0xFFF0
      } else {
        true
      }
    } else {
      false
    }
  });
    
  output
}
