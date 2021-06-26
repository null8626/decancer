use crate::parser::Parser;

pub(crate) fn parse(parser: &mut Parser) -> bool {
  let three_bytes = parser.get_multiple(3);

  if three_bytes[0] == 0xD83D && three_bytes[1] == 0xDD1F {
    // emoji '10'
    parser.push_byte(49);
    parser.push_byte(48);
    
    parser.advance(2);
    return true;
  } else if three_bytes[0] >= 0x30 && three_bytes[0] <= 0x39 && three_bytes[1] == 0x20E3 {
    // emoji '0' to '9'
    
    parser.push_byte(three_bytes[0]);
    parser.advance(2);
    return true;
  } else if three_bytes[0] == 0xD83C && three_bytes[1] >= 0xDDE6 && three_bytes[1] <= 0xDDFF {
    // emoji 'a' to 'z'
    
    parser.push_byte(three_bytes[1] - 0xDD85);
    parser.advance(2);
    return true;
  } else if three_bytes[0] == 0x2757 {
    // emoji '!'
    
    parser.push_byte(33);
    parser.advance(1);
    return true;
  } else if three_bytes[0] == 0x2753 {
    // emoji '?'
    
    parser.push_byte(63);
    parser.advance(1);
    return true;
  } else if three_bytes[1] == 0xFE0F && three_bytes[2] == 0x20E3 {
    // either emojis '#' or '*'
    
    if three_bytes[0] == 0x23 {
      parser.push_byte(35);
      parser.advance(3);
      return true;
    } else if three_bytes[0] == 0x2A {
      parser.push_byte(42);
      parser.advance(3);
      return true;
    }
  }
  
  false
}