// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2021-2026 null8626

use lazy_static::lazy_static;
use regex::bytes::{Regex, RegexBuilder};

macro_rules! re {
  ($pattern:literal) => {{
    Some(
      RegexBuilder::new($pattern)
        .unicode(false)
        .dfa_size_limit(83886080)
        .build()
        .unwrap(),
    )
  }};
}

lazy_static! {
  static ref REGEXES: [Option<Regex>; 26] = [
    re!(r"^(?:\/-*\\|[\[\(\{<]L)"),
    re!(r"^[\\\/\[\]\{\}\(\)\:\|iIljJ17T!](?:3|\-*[\]\)\}>])"),
    None,
    re!(
      r"^(?:[\\\/\[\{\(\:\|iIljJ1T!][\]\}\)>7]|\][\}\)>7]|\}[\]\)>7]|\)[\]\}>7]|7[\]\}\)>]|c[\\\/\[\{\(\:\|iIljJ1T!])"
    ),
    re!(r"^[\(\{\[<cC]\-+"),
    re!(r"^[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]=+"),
    re!(r"^[\(\{\[<cC]_*[\+,\.]"),
    re!(r"^(?:\}\{|[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][\-~\+]+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!])"),
    re!(r"^\]\["),
    re!(r"^[\,\.\+]?_+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]"),
    re!(r"^(?:[\\\/\]\}\)\:\|iIljJ17T!][\(\{\[<cC]|\[[\(\{<cC]|\([\{<cC]|\{[<cC])"),
    re!(r"^[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]_+"),
    re!(
      r"^(?:[\/\[\]\{\}\(\)\:\|iIljJ17T!<](?:[vV]|\\\/)[>\\\[\]\{\}\(\)\:\|iIljJ17T!]|[\/\[\]\{\}\(\)\:\|iIljJ17T!]\\[\/\[\]\{\}\(\)\:\|iIljJ17T!]\\|[\\\[\]\{\}\(\)\:\|iIljJ17T!]\/[\\\[\]\{\}\(\)\:\|iIljJ17T!]\/|[n1^]{2}|rn)"
    ),
    re!(r"^(?:[\/\[\]\{\}\(\)\:\|iIljJ17T!<]\\[>\/\[\]\{\}\(\)\:\|iIljJ17T!]|\/[vV]|^\/)"),
    re!(r"^[\(\{\[<cC][\]\}\)>7]"),
    re!(r"^[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][o0\*^]"),
    re!(r"^(?:[\(\{\[<cC][\]\}\)>7]|[oO0])_+"),
    re!(r"^[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][2Zz`\?]"),
    None,
    re!(r"^[\-~]+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][\-~]+"),
    re!(
      r"^(?:[\\\/\[\]\{\}\(\)\:\|iIljJ17T!<]_+[>\\\/\[\]\{\}\(\)\:\|iIljJ17T!]|L[\\\/\[\]\{\}\(\)\:\|iIljJ17T!])"
    ),
    re!(r"^(?:[\\\[\]\{\}\(\)\:\|iIljJ17T!]\/|\\[\/\[\]\{\}\(\)\:\|iIljJ17T!]|\\\|)"),
    re!(
      r"^(?:[\(\{\[<cC]n[\]\}\)>7]|\\_*(?:\/\\|[^xX\[\]\{\}\(\)\:\|iIljJ17T!])_*\/|[vVuU]{2}|\\N)"
    ),
    re!(r"^[>\}\)\]][<\{\(\[]"),
    re!(r"^`\/"),
    re!(r"^(?:([~\-][\\\/])|[7>])_+"),
  ];
}

pub(super) fn find(haystack: &[u8], character: u32) -> Option<usize> {
  REGEXES[match character {
    65..=90 => character - 65,

    97..=122 => character - 97,

    _ => return None,
  } as usize]
    .as_ref()?
    .find(haystack)
    .map(|mat| mat.len())
}
