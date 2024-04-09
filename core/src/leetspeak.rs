use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

macro_rules! re {
  ($pattern:literal) => {{
    Some(
      RegexBuilder::new(concat!("^(", $pattern, ")"))
        .unicode(false)
        .build()
        .unwrap(),
    )
  }};
}

lazy_static! {
  static ref REGEXES: [Option<Regex>; 26] = [
    re!(r"\/[\\\-]|\[[\]\}\)>7]L"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]3"),
    None,
    re!(r"[\\\/\[\{\(\:\|iIljJ1T!][\]\}\)>7]|\][\}\)>7]|\}[\]\)>7]|\)[\]\}>7]|7[\]\}\)>]"),
    re!(r"[\(\{\[<cC]\-+"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]=+"),
    re!(r"[\(\{\[<cC]_+[\+_,]"),
    re!(r"\}\{|[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][\-~]+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]"),
    re!(r"\]\["),
    re!(r"[\,\.]?_+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]"),
    re!(r"[\\\/\]\}\)\:\|iIljJ17T!][\(\{\[<cC]|\[[\(\{<cC]|\([\{<cC]|\{[<cC]"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]_+"),
    re!(
      r"[\/\[\]\{\}\(\)\:\|iIljJ17T!]([vV]|\\\/)[\\\[\]\{\}\(\)\:\|iIljJ17T!]|[\/\[\]\{\}\(\)\:\|iIljJ17T!]\\[\/\[\]\{\}\(\)\:\|iIljJ17T!]\\|[\\\[\]\{\}\(\)\:\|iIljJ17T!]\/[\\\[\]\{\}\(\)\:\|iIljJ17T!]\/|[n1]{2}|rn"
    ),
    re!(r"[\/\[\]\{\}\(\)\:\|iIljJ17T!]\\[\/\[\]\{\}\(\)\:\|iIljJ17T!]|\/[vV]|\^\/"),
    re!(r"[\(\{\[<cC][\]\}\)>7]"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][o0\*\^]"),
    re!(r"[\(\{\[<cC][\]\}\)>7]_+"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!][2Zz`\?]"),
    None,
    re!(r"\-+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]\-+"),
    re!(r"[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]_+[\\\/\[\]\{\}\(\)\:\|iIljJ17T!]"),
    re!(r"\\\/|\|\/|\\\|"),
    re!(r"[\(\{\[<cC]n[\]\}\)>7]|\\_*(\/\\|[\^xX\[\]\{\}\(\)\:\|iIljJ17T!])_*\/|[vVuU]{2}"),
    re!(r"[\]\}\)>7][\(\{\[<cC]"),
    re!(r"`\/"),
    re!(r"(([~\-][\\\/])|[7>])_+"),
  ];
}

pub(crate) fn find(haystack: &str, character: u32) -> Option<usize> {
  let idx = match character {
    65..=90 => character - 65,
    97..=122 => character - 97,
    _ => return None,
  };

  let regex = &REGEXES[idx as usize];

  regex.as_ref()?.find(haystack).map(|mat| mat.len())
}
