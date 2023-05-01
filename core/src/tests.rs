use crate::{cure, CuredString};
use core::mem::transmute;

#[test]
fn similar_test() {
  let cured = unsafe { transmute::<_, CuredString>(String::from("vwv cvnt 1l1")) };

  assert!(cured.starts_with("uwu"));
  assert!(cured.ends_with("lil"));
  assert!(cured.contains("cunt"));
}

#[test]
fn zalgo_test() {
  assert_eq!(cure("z̸a̸l̸g̷o̶ ̷s̵u̴c̶k̴s̸"), "zalgo sucks");
}

#[test]
fn multi_char_test() {
  assert_eq!(cure("\u{E6}\u{133}\u{153}\u{1C1}\u{1C6}\u{1C9}\u{1CC}\u{1E3}\u{1F3}\u{1FD}\u{238}\u{2A3}\u{2A4}\u{2A6}\u{2A7}\u{2A8}\u{2A9}\u{2AA}\u{2AB}\u{42B}\u{42E}\u{44E}\u{4D5}\u{4F9}\u{1D01}\u{1D02}\u{1D14}\u{1D6B}\u{1D7A}\u{2016}\u{2025}\u{2026}\u{2034}\u{2037}\u{203C}\u{2047}\u{2048}\u{2049}\u{2057}\u{20A8}\u{20B6}\u{2100}\u{2101}\u{2105}\u{2106}\u{2114}\u{2116}\u{211E}\u{2120}\u{2121}\u{2122}\u{213B}\u{214D}\u{214F}\u{2171}\u{2172}\u{2173}\u{2175}\u{2176}\u{2177}\u{2178}\u{217A}\u{217B}\u{221E}\u{2225}\u{226A}\u{226B}\u{22D8}\u{22D9}\u{2381}\u{2382}\u{23E8}\u{2400}\u{2401}\u{2402}\u{2403}\u{2404}\u{2405}\u{2406}\u{2407}\u{2408}\u{2409}\u{240A}\u{240B}\u{240C}\u{240D}\u{240E}\u{240F}\u{2410}\u{2411}\u{2412}\u{2413}\u{2414}\u{2415}\u{2416}\u{2417}\u{2418}\u{2419}\u{241A}\u{241B}\u{241C}\u{241D}\u{241E}\u{241F}\u{2420}\u{2421}\u{2424}\u{244A}\u{2469}\u{246A}\u{246B}\u{246C}\u{246D}\u{246E}\u{246F}\u{2470}\u{2471}\u{2472}\u{2473}\u{247D}\u{247E}\u{247F}\u{2480}\u{2481}\u{2482}\u{2483}\u{2484}\u{2485}\u{2486}\u{2487}\u{2491}\u{2492}\u{2493}\u{2494}\u{2495}\u{2496}\u{2497}\u{2498}\u{2499}\u{249A}\u{249B}\u{24F4}\u{24FE}\u{277F}\u{2789}\u{2793}\u{2A20}\u{2A74}\u{2A75}\u{2A76}\u{2AA5}\u{2AFB}\u{2AFD}\u{2CF9}\u{2E28}\u{2E29}\u{A4FA}\u{A4FB}\u{A4FE}\u{A699}\u{A729}\u{A733}\u{A735}\u{A737}\u{A739}\u{A73B}\u{A73D}\u{A74F}\u{A761}\u{A777}\u{AB63}\u{FB00}\u{FB01}\u{FB02}\u{FB03}\u{FB04}\u{FB06}\u{1F12D}\u{1F12E}\u{1F14A}\u{1F14B}\u{1F14C}\u{1F14D}\u{1F14E}\u{1F14F}\u{1F16A}\u{1F16B}\u{1F16C}\u{1F16D}\u{1F18B}\u{1F18C}\u{1F18D}\u{1F18E}\u{1F18F}\u{1F190}\u{1F191}\u{1F192}\u{1F193}\u{1F194}\u{1F195}\u{1F196}\u{1F197}\u{1F198}\u{1F199}\u{1F19A}\u{1F19B}\u{1F19C}\u{1F19D}\u{1F19E}\u{1F19F}\u{1F1A0}\u{1F1A1}\u{1F1A2}\u{1F1A3}\u{1F1A4}\u{1F1A6}\u{1F1A7}\u{1F1A8}\u{1F1A9}\u{1F1AA}\u{1F1AB}\u{1F1AC}\u{1F51F}\u{1F700}\u{1F707}\u{1F75C}\u{1F76B}\u{1F76C}"), "aeijoeiidzljnjaedzaedbdzd3tstftcfnlslzblio10aebiaeaeaouethll.....''''''!!???!!?\"\"rsltacascoculbnopxsmteltmfaxas000iiiiiivviviiviiiixxixiiooii<<>><<<>>>a...aa10nulsohstxetxeotenqackbelbshtlfvtffcrsosidledc1dc2dc3dc4naksynetbcanemsubescfsgsrsusspdelnl\\\\1011121314151617181920101112131415161718192010111213141516171819202010101010>>::======></////\\\\(())...,-.oobaaaoauavavayoowtpuofffiflffifflstcdwzhvmvsdssppvwcmcmdmrccicpasaabwcdjclcoolfreeidnewngoksosup!vs3d2ndscr2k4k8k5.17.122.260p120phchdrhi-reslosslessshvuhdvod10qearsssmbvb");
}

#[test]
fn fonts_test() {
  assert_eq!(
    cure(
      "\u{1FBF0}\u{1FBF1}\u{1FBF2}\u{1FBF3}\u{1FBF4}\u{1FBF5}\u{1FBF6}\u{1FBF7}\u{1FBF8}\u{1FBF9}"
    ),
    "0123456789"
  );

  assert_eq!(cure("\u{1D51E}\u{1D504}\u{1D586}\u{1D56C}\u{1D4EA}\u{1D4D0}\u{1D4B6}\u{1D49C}\u{1D552}\u{1D538}\u{FF41}\u{FF21}\u{1F130}\u{24D0}\u{24B6}\u{1D41A}\u{1D400}\u{1D5EE}\u{1D5D4}\u{1D622}\u{1D608}\u{1D656}\u{1D63C}\u{1D68A}\u{1D670}\u{1F170}\u{1D6AB}\u{1D6B2}\u{1D6DB}\u{1D6E5}\u{1D6EC}\u{1D715}\u{1D759}\u{1D760}\u{1D789}\u{1D793}\u{1D7C3}"), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
  assert_eq!(cure("\u{1D51F}\u{1D505}\u{1D587}\u{1D56D}\u{1D4EB}\u{1D4D1}\u{1D4B7}\u{1D553}\u{1D539}\u{FF42}\u{FF22}\u{1F131}\u{24D1}\u{24B7}\u{1D41B}\u{1D401}\u{1D5EF}\u{1D5D5}\u{1D623}\u{1D609}\u{1D657}\u{1D63D}\u{1D68B}\u{1D671}\u{1F171}\u{1D6FD}"), "bbbbbbbbbbbbbbbbbbbbbbbbbb");
  assert_eq!(cure("\u{1D520}\u{1D588}\u{1D56E}\u{1D4EC}\u{1D4D2}\u{1D4B8}\u{1D49E}\u{1D554}\u{FF43}\u{FF23}\u{1F132}\u{24D2}\u{24B8}\u{1D41C}\u{1D402}\u{1D5F0}\u{1D5D6}\u{1D624}\u{1D60A}\u{1D658}\u{1D63E}\u{1D68C}\u{1D672}\u{1F172}\u{1D6D3}\u{1D70D}\u{1D781}\u{1D7BB}"), "cccccccccccccccccccccccccccc");
  assert_eq!(cure("\u{1D521}\u{1D507}\u{1D589}\u{1D56F}\u{1D4ED}\u{1D4D3}\u{1D4B9}\u{1D49F}\u{1D555}\u{1D53B}\u{FF44}\u{FF24}\u{1F133}\u{24D3}\u{24B9}\u{1D41D}\u{1D403}\u{1D5F1}\u{1D5D7}\u{1D625}\u{1D60B}\u{1D659}\u{1D63F}\u{1D68D}\u{1D673}\u{1F173}"), "dddddddddddddddddddddddddd");
  assert_eq!(cure("\u{1D522}\u{1D508}\u{1D58A}\u{1D570}\u{1D4EE}\u{1D4D4}\u{1D556}\u{1D53C}\u{FF45}\u{FF25}\u{1F134}\u{24D4}\u{24BA}\u{1D41E}\u{1D404}\u{1D5F2}\u{1D5D8}\u{1D626}\u{1D60C}\u{1D65A}\u{1D640}\u{1D68E}\u{1D674}\u{1F174}\u{1D6BA}\u{1D6C6}\u{1D6DC}\u{1D6F4}\u{1D700}\u{1D709}\u{1D716}\u{1D71A}\u{1D768}\u{1D774}\u{1D78A}\u{1D78E}\u{1D7A2}\u{1D7AE}\u{1D7C4}\u{1D7C8}"), "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
  assert_eq!(cure("\u{1D523}\u{1D509}\u{1D58B}\u{1D571}\u{1D4EF}\u{1D4D5}\u{1D4BB}\u{1D557}\u{1D53D}\u{FF46}\u{FF26}\u{1F135}\u{24D5}\u{24BB}\u{1D41F}\u{1D405}\u{1D5F3}\u{1D5D9}\u{1D627}\u{1D60D}\u{1D65B}\u{1D641}\u{1D68F}\u{1D675}\u{1F175}\u{1D7CB}"), "ffffffffffffffffffffffffff");
  assert_eq!(cure("\u{1D524}\u{1D50A}\u{1D58C}\u{1D572}\u{1D4F0}\u{1D4D6}\u{1D4A2}\u{1D558}\u{1D53E}\u{FF47}\u{FF27}\u{1F136}\u{24D6}\u{24BC}\u{1D420}\u{1D406}\u{1D5F4}\u{1D5DA}\u{1D628}\u{1D60E}\u{1D65C}\u{1D642}\u{1D690}\u{1D676}\u{1F176}"), "ggggggggggggggggggggggggg");
  assert_eq!(cure("\u{1D525}\u{1D58D}\u{1D573}\u{1D4F1}\u{1D4D7}\u{1D4BD}\u{1D559}\u{FF48}\u{FF28}\u{1F137}\u{24D7}\u{24BD}\u{1D421}\u{1D407}\u{1D5F5}\u{1D5DB}\u{1D629}\u{1D60F}\u{1D65D}\u{1D643}\u{1D691}\u{1D677}\u{1F177}"), "hhhhhhhhhhhhhhhhhhhhhhh");
  assert_eq!(cure("\u{1D526}\u{1D58E}\u{1D574}\u{1D4F2}\u{1D4D8}\u{1D4BE}\u{1D55A}\u{1D540}\u{FF49}\u{FF29}\u{1F138}\u{24D8}\u{24BE}\u{1D422}\u{1D408}\u{1D5F6}\u{1D5DC}\u{1D62A}\u{1D610}\u{1D65E}\u{1D644}\u{1D692}\u{1D678}\u{1F178}\u{1D6CA}\u{1D704}\u{1D778}\u{1D7B2}"), "iiiiiiiiiiiiiiiiiiiiiiiiiiii");
  assert_eq!(cure("\u{1D527}\u{1D50D}\u{1D58F}\u{1D575}\u{1D4F3}\u{1D4D9}\u{1D4BF}\u{1D4A5}\u{1D55B}\u{1D541}\u{FF4A}\u{FF2A}\u{1F139}\u{24D9}\u{24BF}\u{1D423}\u{1D409}\u{1D5F7}\u{1D5DD}\u{1D62B}\u{1D611}\u{1D65F}\u{1D645}\u{1D693}\u{1D679}\u{1F179}"), "jjjjjjjjjjjjjjjjjjjjjjjjjj");
  assert_eq!(cure("\u{1D528}\u{1D50E}\u{1D590}\u{1D576}\u{1D4F4}\u{1D4DA}\u{1D4C0}\u{1D4A6}\u{1D55C}\u{1D542}\u{FF4B}\u{FF2B}\u{1F13A}\u{24DA}\u{24C0}\u{1D424}\u{1D40A}\u{1D5F8}\u{1D5DE}\u{1D62C}\u{1D612}\u{1D660}\u{1D646}\u{1D694}\u{1D67A}\u{1F17A}\u{1D6CB}\u{1D705}\u{1D779}\u{1D7B3}"), "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
  assert_eq!(cure("\u{1D529}\u{1D50F}\u{1D591}\u{1D577}\u{1D4F5}\u{1D4DB}\u{1D4C1}\u{1D55D}\u{1D543}\u{FF4C}\u{FF2C}\u{1F13B}\u{24DB}\u{24C1}\u{1D425}\u{1D40B}\u{1D5F9}\u{1D5DF}\u{1D62D}\u{1D613}\u{1D661}\u{1D647}\u{1D695}\u{1D67B}\u{1F17B}"), "lllllllllllllllllllllllll");
  assert_eq!(cure("\u{1D52A}\u{1D510}\u{1D592}\u{1D578}\u{1D4F6}\u{1D4DC}\u{1D4C2}\u{1D55E}\u{1D544}\u{FF4D}\u{FF2D}\u{1F13C}\u{24DC}\u{24C2}\u{1D426}\u{1D40C}\u{1D5FA}\u{1D5E0}\u{1D62E}\u{1D614}\u{1D662}\u{1D648}\u{1D696}\u{1D67C}\u{1F17C}"), "mmmmmmmmmmmmmmmmmmmmmmmmm");
  assert_eq!(cure("\u{1D52B}\u{1D511}\u{1D593}\u{1D579}\u{1D4F7}\u{1D4DD}\u{1D4C3}\u{1D4A9}\u{1D55F}\u{FF4E}\u{FF2E}\u{1F13D}\u{24DD}\u{24C3}\u{1D427}\u{1D40D}\u{1D5FB}\u{1D5E1}\u{1D62F}\u{1D615}\u{1D663}\u{1D649}\u{1D697}\u{1D67D}\u{1F17D}\u{1D6B7}\u{1D6C8}\u{1D6F1}\u{1D702}\u{1D765}\u{1D776}\u{1D79F}\u{1D7B0}"), "nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn");
  assert_eq!(cure("\u{1D52C}\u{1D512}\u{1D594}\u{1D57A}\u{1D4F8}\u{1D4DE}\u{1D4AA}\u{1D560}\u{1D546}\u{FF4F}\u{FF2F}\u{1F13E}\u{24DE}\u{24C4}\u{1D428}\u{1D40E}\u{1D5FC}\u{1D5E2}\u{1D630}\u{1D616}\u{1D664}\u{1D64A}\u{1D698}\u{1D67E}\u{1F17E}\u{1D6AF}\u{1D6C9}\u{1D6D0}\u{1D6E9}\u{1D703}\u{1D75D}\u{1D767}\u{1D777}\u{1D7B1}\u{1D797}"), "ooooooooooooooooooooooooooooooooooo");
  assert_eq!(cure("\u{1D52D}\u{1D513}\u{1D595}\u{1D57B}\u{1D4F9}\u{1D4DF}\u{1D4C5}\u{1D4AB}\u{1D561}\u{FF50}\u{FF30}\u{1F13F}\u{24DF}\u{24C5}\u{1D429}\u{1D40F}\u{1D5FD}\u{1D5E3}\u{1D631}\u{1D617}\u{1D665}\u{1D64B}\u{1D699}\u{1D67F}\u{1F17F}\u{1D6D2}\u{1D70C}\u{1D780}\u{1D7BA}"), "ppppppppppppppppppppppppppppp");
  assert_eq!(cure("\u{1D52E}\u{1D514}\u{1D596}\u{1D57C}\u{1D4FA}\u{1D4E0}\u{1D4C6}\u{1D4AC}\u{1D562}\u{FF51}\u{FF31}\u{1F140}\u{24E0}\u{24C6}\u{1D42A}\u{1D410}\u{1D5FE}\u{1D5E4}\u{1D632}\u{1D618}\u{1D666}\u{1D64C}\u{1D69A}\u{1D680}\u{1F180}"), "qqqqqqqqqqqqqqqqqqqqqqqqq");
  assert_eq!(cure("\u{1D52F}\u{1D597}\u{1D57D}\u{1D4FB}\u{1D4E1}\u{1D4C7}\u{1D563}\u{FF52}\u{FF32}\u{1F141}\u{24E1}\u{24C7}\u{1D42B}\u{1D411}\u{1D5FF}\u{1D5E5}\u{1D633}\u{1D619}\u{1D667}\u{1D64D}\u{1D69B}\u{1D681}\u{1F181}\u{1D6AA}\u{1D6E4}\u{1D758}\u{1D792}"), "rrrrrrrrrrrrrrrrrrrrrrrrrrr");
  assert_eq!(cure("\u{1D530}\u{1D516}\u{1D598}\u{1D57E}\u{1D4FC}\u{1D4E2}\u{1D4C8}\u{1D4AE}\u{1D564}\u{1D54A}\u{FF53}\u{FF33}\u{1F142}\u{24E2}\u{24C8}\u{1D42C}\u{1D412}\u{1D600}\u{1D5E6}\u{1D634}\u{1D61A}\u{1D668}\u{1D64E}\u{1D69C}\u{1D682}\u{1F182}"), "ssssssssssssssssssssssssss");
  assert_eq!(cure("\u{1D531}\u{1D517}\u{1D599}\u{1D57F}\u{1D4FD}\u{1D4E3}\u{1D4C9}\u{1D4AF}\u{1D565}\u{1D54B}\u{FF54}\u{FF34}\u{1F143}\u{24E3}\u{24C9}\u{1D42D}\u{1D413}\u{1D601}\u{1D5E7}\u{1D635}\u{1D61B}\u{1D669}\u{1D64F}\u{1D69D}\u{1D683}\u{1F183}\u{1D6D5}\u{1D70F}\u{1D7BD}"), "ttttttttttttttttttttttttttttt");
  assert_eq!(cure("\u{1D532}\u{1D518}\u{1D59A}\u{1D580}\u{1D4FE}\u{1D4E4}\u{1D4CA}\u{1D4B0}\u{1D566}\u{1D54C}\u{FF55}\u{FF35}\u{1F144}\u{24E4}\u{24CA}\u{1D42E}\u{1D414}\u{1D602}\u{1D5E8}\u{1D636}\u{1D61C}\u{1D66A}\u{1D650}\u{1D69E}\u{1D684}\u{1F184}\u{1D6D6}"), "uuuuuuuuuuuuuuuuuuuuuuuuuuu");
  assert_eq!(cure("\u{1D533}\u{1D519}\u{1D59B}\u{1D581}\u{1D4FF}\u{1D4E5}\u{1D4CB}\u{1D4B1}\u{1D567}\u{1D54D}\u{FF56}\u{FF36}\u{1F145}\u{24E5}\u{24CB}\u{1D42F}\u{1D415}\u{1D603}\u{1D5E9}\u{1D637}\u{1D61D}\u{1D66B}\u{1D651}\u{1D69F}\u{1D685}\u{1F185}\u{1D6C1}\u{1D6CE}\u{1D6FB}\u{1D708}\u{1D76F}\u{1D77C}\u{1D7A9}\u{1D7B6}"), "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv");
  assert_eq!(cure("\u{1D534}\u{1D51A}\u{1D59C}\u{1D582}\u{1D500}\u{1D4E6}\u{1D4CC}\u{1D4B2}\u{1D568}\u{1D54E}\u{FF57}\u{FF37}\u{1F146}\u{24E6}\u{24CC}\u{1D430}\u{1D416}\u{1D604}\u{1D5EA}\u{1D638}\u{1D61E}\u{1D66C}\u{1D652}\u{1D6A0}\u{1D686}\u{1F186}\u{1D6DA}\u{1D714}\u{1D788}\u{1D7C2}"), "wwwwwwwwwwwwwwwwwwwwwwwwwwwwww");
  assert_eq!(cure("\u{1D535}\u{1D51B}\u{1D59D}\u{1D583}\u{1D501}\u{1D4E7}\u{1D4CD}\u{1D4B3}\u{1D569}\u{1D54F}\u{FF58}\u{FF38}\u{1F147}\u{24E7}\u{24CD}\u{1D431}\u{1D417}\u{1D605}\u{1D5EB}\u{1D639}\u{1D61F}\u{1D66D}\u{1D653}\u{1D6A1}\u{1D687}\u{1F187}\u{1D6D8}\u{1D6DE}\u{1D718}\u{1D786}\u{1D78C}\u{1D7C0}\u{1D7C6}"), "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
  assert_eq!(cure("\u{1D536}\u{1D51C}\u{1D59E}\u{1D584}\u{1D502}\u{1D4E8}\u{1D4CE}\u{1D4B4}\u{1D56A}\u{1D550}\u{FF59}\u{FF39}\u{1F148}\u{24E8}\u{24CE}\u{1D432}\u{1D418}\u{1D606}\u{1D5EC}\u{1D63A}\u{1D620}\u{1D66E}\u{1D654}\u{1D6A2}\u{1D688}\u{1F188}\u{1D6BC}\u{1D6C4}\u{1D6F6}\u{1D7AC}"), "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");
  assert_eq!(cure("\u{1D537}\u{1D59F}\u{1D585}\u{1D503}\u{1D4E9}\u{1D4CF}\u{1D4B5}\u{1D56B}\u{FF5A}\u{FF3A}\u{1F149}\u{24E9}\u{24CF}\u{1D433}\u{1D419}\u{1D607}\u{1D5ED}\u{1D63B}\u{1D621}\u{1D66F}\u{1D655}\u{1D6A3}\u{1D689}\u{1F189}"), "zzzzzzzzzzzzzzzzzzzzzzzz");
}
