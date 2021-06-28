// the dumping ground.
// beware, lots of unicode characters.

const startRegex = /[\uD800-\uDB7F\u030d\u030e\u0304\u0305\u033f\u0311\u0306\u0310\u0352\u0357\u0351\u0307\u0308\u030a\u0342\u0343\u0344\u034a\u034b\u034c\u0303\u0302\u030c\u0350\u0300\u0301\u030b\u030f\u0312\u0313\u0314\u033d\u0309\u0363\u0364\u0365\u0366\u0367\u0368\u0369\u036a\u036b\u036c\u036d\u036e\u036f\u033e\u035b\u0346\u031a\u0315\u031b\u0340\u0341\u0358\u0321\u0322\u0327\u0328\u0334\u0335\u0336\u034f\u035c\u035d\u035e\u035f\u0360\u0362\u0338\u0337\u0361\u0489\u0316\u0317\u0318\u0319\u031c\u031d\u031e\u031f\u0320\u0324\u0325\u0326\u0329\u032a\u032b\u032c\u032d\u032e\u032f\u0330\u0331\u0332\u0333\u0339\u033a\u033b\u033c\u0345\u0347\u0348\u0349\u034d\u034e\u0353\u0354\u0355\u0356\u0359\u035a\u0323]/g;

const alphabeticalStyles = [
    120094, 120068, 120198, 120172,
    120042, 120016, 119990, 119964,
    120146, 120120,  65345,  65313,
    127280,   9424,   9398, 119834,
    119808, 120302, 120276, 120354,
    120328, 120406, 120380, 120458,
    120432, 127344
];
  
const numericalStyles = [
    120802, 120792,
     65296,   8320,
      8304,   9450,
    120782, 120812,
    120822
];

const miscAlphabetical = [
    '\u00e0\u00e1\u00e2\u00e3\u00e4\u00e5\u0101\u0103\u0105\u0250\u0251\u0252\u03ac\u03b1\u0430\u0434\u0467\u04d1\u04d3\u15c5\u1d00\u1d2c\u1d43\u1d44\u2090\u20b3\u237a\u4e39\u5342\ua4ee\uab7a\udc34\udc4e\udc68\udc82\udda0\uddba\udea8\udec2\udee2\udefc\udf1c\udf36\udf40\udf56\udf70\udf90\udfaa', // a
    '\u0180\u0183\u0185\u0253\u0299\u03b2\u0431\u0432\u044a\u044c\u0463\u048d\u0495\u0e3f\u13fc\u1472\u15af\u15f7\u1d03\u1d2e\u1d2f\u1d47\u1d5d\u1d66\u1d6c\u212c\u4e43\u65e5\ua4d0\ua7b5\uab9f\udc35\udc4f\udc69\udc83\udda1\uddbb\udea9\udee3\udf01\udf1d\udf57\udf91', // b
    '\u00a9\u00e7\u0107\u0109\u010b\u010d\u0188\u023c\u0255\u0297\u037b\u037c\u037d\u0441\u0481\u04ab\u1103\u1d04\u1d9c\u20b5\u2102\u212d\u217d\u2ca5\u4ea1\u531a\ua4da\uabaf\udc36\udc50\udc6a\udc84\udda2\uddbc\udf02\udf4c', // c
    '\u0111\u018c\u0501\u146f\u15de\u15ea\u1d05\u1d06\u1d30\u1d48\u1d5f\u1d6d\u2145\u2146\u217e\u53e5\ua4d2\ua4d3\uab70\uabb7\udc37\udc51\udc6b\udc85\udda3\uddbd', // d
    '\u00e8\u00e9\u00ea\u00eb\u0113\u0115\u0117\u0119\u011b\u01b9\u01dd\u0205\u0207\u021d\u0221\u0229\u0247\u0256\u0257\u0283\u03ad\u03b5\u03be\u03f1\u03f2\u03f5\u03f6\u0435\u0450\u0451\u0454\u0465\u04bd\u04bf\u04d7\u04d9\u04db\u15f4\u1d07\u1d31\u1d49\u1d4b\u2091\u2094\u20ac\u212e\u212f\u2130\u2147\u22ff\u2d39\u30e8\u4e47\ua4f0\uab32\uab7c\udc38\udc52\udc6c\udc86\udda4\uddbe\udeac\udee6\udf20\udf5a\udf94', // e
    '\u010f\u0192\u0258\u0259\u025a\u025b\u025c\u025d\u025e\u03dd\u0493\u04fb\u0584\u15b4\u1d6e\u1da0\u1e9d\u20a3\u2131\u4e4d\u5343\ua4dd\ua799\uab35\udc39\udc53\udc6d\udc87\udda5\uddbf\udea5\udfca', // f
    '\u011d\u011f\u0121\u0123\u01e5\u01e7\u01f5\u0260\u0261\u0262\u0265\u0266\u0267\u029b\u050d\u0581\u13fb\u1d33\u1d4d\u1d77\u1d79\u1d83\u20b2\u210a\u5442\ua4d6\uab86\uab90\udc3a\udc54\udc6e\udc88\udda6\uddc0', // g
    '\u0125\u0127\u0195\u021f\u029c\u02b0\u043d\u045b\u04a3\u04a5\u04bb\u04c8\u04ca\u0570\u157c\u1d34\u2095\u210b\u210c\u210d\u210e\u2c68\u2c8f\u5344\u5efe\ua4e7\uab8b\uab92\udc3b\udc6f\udc89\udda7\uddc1\udeae\udecf\udee8\udf22\udf5c\udf96', // h
    '\u00ec\u00ed\u00ee\u00ef\u0129\u012b\u012d\u012f\u0131\u01d0\u0209\u020b\u0268\u0269\u026a\u02db\u037a\u03af\u03b9\u03ca\u0456\u0457\u04cf\u05c0\u05d5\u05df\u0627\u07ca\u16c1\u1d09\u1d35\u1d4e\u1d62\u1d7b\u1d7c\u1da6\u2071\u2110\u2111\u2139\u2148\u2170\u217c\u2223\u2373\u23fd\u2c93\u2d4f\u4e28\u5de5\ua4f2\ua647\uab75\udc3c\udc56\udc59\udc70\udc8a\udda8\uddc2\uddc5\udea4\udeb0\udeca\udeea\udf04\udf24\udf3e\udf5e\udf78\udf98\udfb2\ufe8d\uffe8', // i
    '\u006a\u0135\u01f0\u0237\u0249\u025f\u0279\u027a\u027b\u029d\u02b2\u03f3\u0458\u148d\u1d0a\u1d36\u2149\u52f9\ua4d9\uab7b\udc3d\udc57\udc71\udc8b\udda9\uddc3\uff8c', // j
    '\u0137\u0138\u0199\u01e9\u029e\u03ba\u03d7\u043a\u045c\u049b\u049d\u049f\u04a1\u16d5\u1d0b\u1d37\u1d4f\u2096\u20ad\u2c95\u7247\ua4d7\uabb6\udc3e\udc58\udc72\udc8c\uddaa\uddc4\udeb1\udeeb\udf25\udf5f\udf99', // k
    '\u007c\u013a\u013c\u013e\u0140\u0142\u017f\u019a\u01c0\u01c1\u0234\u026b\u026c\u026d\u029f\u02e1\u0661\u06f1\u1102\u14aa\u1d0c\u1d38\u2097\u2112\u2113\u2c61\u2cd1\u3057\u3125\ua4e1\uabae\udc3f\udc73\udc8d\uddab\udf16\ufe8e', // l
    '\u0271\u03bc\u03fb\u043c\u04ce\u15f0\u16d6\u1d0d\u1d1f\u1d39\u1d50\u1d5a\u2098\u20a5\u2133\u217f\u2c99\u518a\u722a\ua4df\uab87\udc40\udc5a\udc74\udc8e\uddac\uddc6\udeb3\udeed\udf00\udf11\udf27\udf61\udf9b', // m
    '\u00f1\u0144\u0146\u0148\u0149\u014b\u014d\u014f\u0151\u019e\u01f9\u0235\u0272\u0273\u0274\u0377\u03ae\u03b7\u0438\u0439\u043f\u045d\u048b\u04e3\u04e5\u0578\u057c\u1d0e\u1d3a\u1d3b\u1d70\u207f\u2099\u20a6\u2115\u2c9b\u51e0\ua4e0\udc41\udc5b\udc75\udc8f\uddad\uddc7\udeb4\udeee\udf28\udf62\udf9c', // n
    '\u00f0\u00f2\u00f3\u00f4\u00f5\u00f6\u00f8\u018d\u01a1\u01a3\u01d2\u01eb\u01ed\u01ff\u020d\u020f\u0223\u022b\u022d\u022f\u0231\u0275\u0276\u0277\u0278\u0298\u03b8\u03bf\u03c3\u03cc\u03d9\u043e\u0473\u047b\u047d\u04e7\u04e9\u04eb\u0585\u05e1\u0647\u0665\u06be\u06c1\u06d5\u06f5\u07c0\u0966\u09e6\u0a66\u0ae6\u0b20\u0b66\u0be6\u0c02\u0c66\u0c82\u0ce6\u0d02\u0d20\u0d66\u0d82\u0e50\u0ed0\u101d\u1040\u10ff\u12d0\u1d0f\u1d10\u1d11\u1d12\u1d13\u1d16\u1d17\u1d3c\u1d52\u1d53\u1d54\u1d55\u200e\u2092\u2134\u2c9f\u2d54\u3007\u3116\u56de\ua4f3\uab3d\udc42\udc5c\udc76\udc90\uddae\uddc8\udeab\udeb6\uded0\uded4\udef0\udf0a\udf0e\udf2a\udf44\udf48\udf64\udf7e\udf82\udf9e\udfb8\udfbc\ufba6\ufba7\ufba8\ufba9\ufbaa\ufbab\ufbac\ufbad\ufee9\ufeea\ufeeb\ufeec', // o
    '\u01a5\u01bf\u03c1\u03f8\u0440\u146d\u1d18\u1d3e\u1d56\u1d71\u1d7d\u209a\u20b1\u2119\u2374\u2ca3\u5369\u5c38\ua4d1\uabb2\udc43\udc5d\udc77\udc91\uddaf\uddc9\udeb8\uded2\udee0\udef2\udf0c\udf1a\udf2c\udf46\udf54\udf66\udf80\udf8e\udfa0\udfba\udfc8', // p
    '\u0239\u024b\u02a0\u051b\u0563\u0566\u1d60\u211a\u2d55\u7532\udc44\udc5e\udc78\udc92\uddb0\uddca', // q
    '\u0155\u0157\u0159\u0211\u0213\u024d\u027c\u027d\u027e\u027f\u0280\u0281\u02b3\u044f\u0453\u0491\u04f7\u1587\u1d19\u1d1a\u1d26\u1d3f\u1d63\u1d72\u1d73\u211b\u211c\u211d\u2c85\u5c3a\ua4e3\uab47\uab48\uab71\uab81\uaba2\udc45\udc5f\udc79\udc93\uddb1\uddcb\udf35', // r
    '\u015b\u015d\u015f\u0161\u01a8\u0219\u023f\u0282\u02e2\u0455\u057f\u1d74\u209b\u20b4\u4e02\u5df1\ua4e2\ua731\uaba5\uabaa\udc46\udc60\udc7a\udc94\uddb2\uddcc\udf3a', // s
    '\u0163\u0165\u0167\u01ab\u01ad\u021b\u0236\u0287\u0288\u0373\u03c4\u03ef\u0442\u04ad\u1d1b\u1d40\u1d57\u2020\u209c\u20ae\u22a4\u27d9\u2ca7\u3112\u535e\ua4d4\uab72\udc47\udc61\udc7b\udc95\uddb3\uddcd\udebb\udef5\udf15\udf2f\udf68\udf69\udfa3', // t
    '\u00f9\u00fa\u00fb\u00fc\u0169\u016b\u016d\u016f\u0171\u0173\u01b0\u01d4\u01d6\u01d8\u01da\u01dc\u0215\u0217\u0289\u028a\u028b\u03b0\u03c5\u03cb\u0433\u0446\u057d\u1200\u144c\u1d1c\u1d1d\u1d1e\u1d41\u1d58\u1d59\u1d64\u1d7e\u1d7f\u222a\u22c3\u3129\u51f5\ua4f4\ua79f\uab4e\uab52\udc48\udc62\udc7c\udc96\uddb4\uddce\uded6\udf10\udf4a\udf84\udfbe', // u
    '\u028c\u03bd\u0475\u0477\u05d8\u0667\u06f7\u142f\u1d20\u1d5b\u1d65\u2174\u2228\u22c1\u2d38\u30ec\ua4e6\ua6df\uaba9\udc49\udc63\udc7d\udc97\uddb5\uddcf\udece\udf06\udf08\udf42\udf7c\udfb6', // v
    '\u0175\u026f\u0270\u028d\u02ac\u02b7\u0448\u0449\u0461\u047f\u051d\u0561\u1d21\u1d42\u20a9\u5c71\ua4ea\uab83\uaba4\udc4a\udc64\udc7e\udc98\uddb6\uddd0\udf0f', // w
    '\u00d7\u02e3\u03c7\u03f0\u0436\u0445\u04b3\u04fd\u04ff\u1541\u157d\u166d\u166e\u16b7\u2093\u2179\u2573\u292b\u292c\u2a2f\u2cad\u2d5d\u30e1\u4e42\ua4eb\uab53\udc4b\udc65\udc7f\udc99\uddb7\uddd1\udebe\udef8\udf17\udf32\udf6c\udfa6', // x
    '\u00fd\u00ff\u0177\u01b4\u0233\u024f\u0263\u0264\u028e\u028f\u02b8\u03b3\u03bb\u03d2\u03d3\u03d4\u0443\u04af\u04b1\u04ef\u04f1\u04f3\u04f5\u10e7\u1d8c\u1eff\u213d\u2ca9\u3068\u311a\ua4ec\uab5a\uab79\uab8d\udc4c\udc66\udc80\udc9a\uddb8\uddd2\udeb2\udebc\udec4\udef6\udefe\udf30\udf38\udf43\udf6a\udf72\udfa4\udfac', // y
    '\u017a\u017c\u017e\u01b6\u0225\u0240\u0290\u0291\u03b6\u1d22\u1d76\u1dbb\u2124\u2128\u2c6c\u4e59\ua4dc\uab93\udc4d\udc67\udc81\udc9b\uddb9\uddd3\udead\udee7\udf21\udf5b\udf95' // z
];

// case sensitive.
const miscOthers = {
    a: '\u039b', // capital lambda,
    d: '\u00d0', // capital eth,
    e: '\u03a3', // capital sigma
    h: '\u0389', // capital eta with tonos
    0: '\u2070\uff10',
    1: '\u00b9\uff11',
    2: '\u00b2\u03e9\u14bf\ua645\ua6ef\ua75b\uff12',
    3: '\u00b3\u018e\u01ba\u01ef\u0292\u0437\u044d\u0499\u04df\u04e1\u04ed\u1d08\u1d23\u1d32\u1d4c\u2ccd\ua76b\udf3b\uff13',
    4: '\u2074\uab9e\uff14',
    5: '\u01bd\u2075\uff15',
    6: '\u2076\u2cd3\uabbe\uff16',
    7: '\u2077\uff17',
    8: '\u09ea\u0a6a\u0b03\u2078\uff18',
    9: '\u09ed\u0a67\u0b68\u0d6d\u2079\u2ccb\ua76f\uff19',
    '>': '\u02c3\u1433\u203a\u276f\uff1e',
    '+': '\u16ed\u207a\u208a\u2795\uff0b',
    '-': '\u02d7\u06d4\u2010\u2011\u2012\u2043\u207b\u208b\u2796\u2cbb\ufe58\uff0d',
    '=': '\u1400\u207c\u208c\u2e40\u30a0\ua4ff\uff1d',
    '(': '\u207d\u208d\u2768\u2772\u3014\ufd3e\uff08\uff5f',
    ')': '\u207e\u208e\u2769\u2773\u29f4\u3015\ufd3f\uff09\uff60',
    '!': '\u01c3\u2d51\uff01',
    '"': '\u201c\u201d\uff02',
    '#': '\uff03',
    '$': '\uff04',
    '%': '\uff05',
    '&': '\ua778\uff06',
    '\'': '\uff07',
    ',': '\u00b8\u060d\u066b\ua4f9\uff0c',
    '.': '\u0660\u06f0\u0701\u0702\u2024\uff0e',
    '/': '\u1735\u2041\u2044\u2215\u2571\u2cc7\u2f03\u3033\u30ce\u31d3\u4e3f\uff0f',
    ':': '\u02d0\u02f8\u0589\u05c3\u0703\u0704\u0903\u0a83\u16ec\u1803\u1809\u205a\u2236\ua4fd\ua789\ufe30\uff1a',
    ';': '\uff1b',
    '<': '\u02c2\u1438\u16b2\u2039\u276e\uff1c',
    '?': '\u0242\u0294\u097d\ua6eb\uab7e\uff1f',
    '@': '\uff20',
    '[': '\u3010\uff3b',
    '\\': '\u2216\u27cd\u29f5\u29f9\u2f02\u31d4\u4e36\ufe68\uff3c',
    ']': '\u3011\uff3d',
    '^': '\u02c4\u02c6\uff3e',
    '_': '\u07fa\ufe4d\ufe4e\ufe4f\uff3f',
    '`': '\uff40',
    '{': '\u2774\uff5b',
    '|': '\uff5c',
    '}': '\u2775\uff5d',
    '~': '\uff5e',
    '*': '\ua60e'
};

const emojis = {
    10: '\ud83d\udd1f',
    0: '\u0030\u20e3', 1: '\u0031\u20e3',
    2: '\u0032\u20e3', 3: '\u0033\u20e3',
    4: '\u0034\u20e3', 5: '\u0035\u20e3',
    6: '\u0036\u20e3', 7: '\u0037\u20e3',
    8: '\u0038\u20e3', 9: '\u0039\u20e3',
    a: '\ud83c\udde6',
    b: '\ud83c\udde7', c: '\ud83c\udde8',
    d: '\ud83c\udde9', e: '\ud83c\uddea',
    f: '\ud83c\uddeb', g: '\ud83c\uddec',
    h: '\ud83c\udded', i: '\ud83c\uddee',
    j: '\ud83c\uddef', k: '\ud83c\uddf0',
    l: '\ud83c\uddf1', m: '\ud83c\uddf2',
    n: '\ud83c\uddf3', o: '\ud83c\uddf4',
    p: '\ud83c\uddf5', q: '\ud83c\uddf6',
    r: '\ud83c\uddf7', s: '\ud83c\uddf8',
    t: '\ud83c\uddf9', u: '\ud83c\uddfa',
    v: '\ud83c\uddfb', w: '\ud83c\uddfc',
    x: '\ud83c\uddfd', y: '\ud83c\uddfe',
    z: '\ud83c\uddff',
    '!': '\u2757', '?': '\u2753',
    '#': '\u0023\ufe0f\u20e3', '*': '\\\u002a\ufe0f\u20e3'
};

module.exports = {
    startRegex,
    alphabeticalStyles,
    numericalStyles,
    miscAlphabetical,
    miscOthers,
    emojis
};