const ALPHABETICAL_1: [&'static [u16]; 26] = [
  &[0x00AA, 0x00E0, 0x00E1, 0x00E2, 0x00E3, 0x00E4, 0x00E5, 0x0101, 0x0103, 0x0105, 0x01CE, 0x01DF, 0x01E1, 0x01FB, 0x0201, 0x0203, 0x0227, 0x0250, 0x0251, 0x0252, 0x03AC, 0x03B1, 0x0430, 0x0434, 0x0467, 0x04D1, 0x04D3, 0x15C5, 0x15E9, 0x1D00, 0x1D2C, 0x1D43, 0x1D44, 0x1D8F, 0x1DF2, 0x1E01, 0x1E9A, 0x1EA1, 0x1EA3, 0x1EA5, 0x1EA7, 0x1EA9, 0x1EAB, 0x1EAD, 0x1EAF, 0x1EB1, 0x1EB3, 0x1EB5, 0x1EB7, 0x1F00, 0x1F01, 0x1F02, 0x1F03, 0x1F04, 0x1F05, 0x1F06, 0x1F07, 0x1F70, 0x1F71, 0x1F80, 0x1F81, 0x1F82, 0x1F83, 0x1F84, 0x1F85, 0x1F86, 0x1F87, 0x1FB0, 0x1FB1, 0x1FB2, 0x1FB3, 0x1FB4, 0x1FB6, 0x1FB7, 0x2090, 0x20B3, 0x237A, 0x2C65, 0x4E39, 0x5342, 0xA4EE, 0xAB31, 0xAB64, 0xAB7A],
  &[0x00DF, 0x0180, 0x0183, 0x0185, 0x0253, 0x0284, 0x0299, 0x03B2, 0x0432, 0x044A, 0x044C, 0x0463, 0x048D, 0x0495, 0x0E3F, 0x13FC, 0x1472, 0x15AF, 0x15F7, 0x16B2, 0x1D03, 0x1D2E, 0x1D2F, 0x1D47, 0x1D5D, 0x1D66, 0x1D6C, 0x1D80, 0x1E03, 0x1E05, 0x1E07, 0x212C, 0x4E43, 0x65E5, 0xA4D0, 0xA797, 0xA7B5, 0xAB9F],
  &[0x00A2, 0x00A9, 0x00E7, 0x0107, 0x0109, 0x010B, 0x010D, 0x0188, 0x023C, 0x0254, 0x0255, 0x0297, 0x037B, 0x037C, 0x037D, 0x03F2, 0x0441, 0x0481, 0x04AB, 0x1103, 0x1455, 0x1D04, 0x1D9C, 0x1DD7, 0x1E09, 0x20B5, 0x2102, 0x2103, 0x212D, 0x217D, 0x2CA5, 0x4EA1, 0x531A, 0xA4DA, 0xA793, 0xA794, 0xABAF, 0xFFE0],
  &[0x0111, 0x018C, 0x0256, 0x0501, 0x146F, 0x15DE, 0x15EA, 0x1D05, 0x1D06, 0x1D30, 0x1D48, 0x1D5F, 0x1D6D, 0x1D81, 0x1D91, 0x1E0B, 0x1E0D, 0x1E0F, 0x1E11, 0x1E13, 0x2145, 0x2146, 0x217E, 0x53E5, 0xA4D2, 0xA4D3, 0xA7C8, 0xAB70, 0xABB7],
  &[0x00A3, 0x00E8, 0x00E9, 0x00EA, 0x00EB, 0x0113, 0x0115, 0x0117, 0x0119, 0x011B, 0x01B9, 0x01DD, 0x0205, 0x0207, 0x0221, 0x0229, 0x0247, 0x0257, 0x0283, 0x03AD, 0x03B5, 0x03BE, 0x03F1, 0x03F5, 0x03F6, 0x0435, 0x0450, 0x0451, 0x0454, 0x0465, 0x04BD, 0x04BF, 0x04D7, 0x04D9, 0x04DB, 0x15F4, 0x1D07, 0x1D31, 0x1D49, 0x1D4B, 0x1D92, 0x1E15, 0x1E17, 0x1E19, 0x1E1B, 0x1E1D, 0x1EB9, 0x1EBB, 0x1EBD, 0x1EBF, 0x1EC1, 0x1EC3, 0x1EC5, 0x1EC7, 0x1F10, 0x1F11, 0x1F12, 0x1F13, 0x1F14, 0x1F15, 0x1F72, 0x1F73, 0x2091, 0x2094, 0x20AC, 0x2107, 0x2108, 0x212E, 0x212F, 0x2130, 0x2140, 0x2147, 0x22FF, 0x2C78, 0x2D39, 0x30E8, 0x4E47, 0xA4F0, 0xAB32, 0xAB34, 0xAB7C, 0xFFE1],
  &[0x010F, 0x017F, 0x0192, 0x0258, 0x0259, 0x025A, 0x025B, 0x025D, 0x025E, 0x03DD, 0x0493, 0x04FB, 0x0584, 0x15B4, 0x1D6E, 0x1D82, 0x1DA0, 0x1E1F, 0x1E9B, 0x1E9C, 0x1E9D, 0x20A3, 0x2109, 0x2131, 0x214E, 0x4E4D, 0x5343, 0xA4DD, 0xA730, 0xA799, 0xAB35],
  &[0x011D, 0x011F, 0x0121, 0x0123, 0x01E5, 0x01E7, 0x01F5, 0x0260, 0x0261, 0x0262, 0x0265, 0x0266, 0x0267, 0x029B, 0x050D, 0x0581, 0x13FB, 0x1D33, 0x1D4D, 0x1D77, 0x1D79, 0x1D83, 0x1E21, 0x20B2, 0x210A, 0x5442, 0xA4D6, 0xA7A1, 0xAB86, 0xAB90],
  &[0x0125, 0x0127, 0x0195, 0x021F, 0x029C, 0x02AE, 0x02AF, 0x02B0, 0x02B1, 0x043D, 0x0452, 0x045B, 0x04A3, 0x04A5, 0x04BB, 0x04C8, 0x04CA, 0x0570, 0x157C, 0x1D34, 0x1E23, 0x1E25, 0x1E27, 0x1E29, 0x1E2B, 0x1E96, 0x2095, 0x210B, 0x210C, 0x210D, 0x210E, 0x210F, 0x2C68, 0x2C8F, 0x5344, 0x5EFE, 0xA4E7, 0xA795, 0xAB8B, 0xAB92],
  &[0x00EC, 0x00ED, 0x00EE, 0x00EF, 0x0129, 0x012B, 0x012D, 0x012F, 0x0131, 0x01D0, 0x0209, 0x020B, 0x0268, 0x0269, 0x026A, 0x02DB, 0x037A, 0x03AF, 0x03B9, 0x03CA, 0x0456, 0x0457, 0x04CF, 0x05C0, 0x05D5, 0x05DF, 0x0627, 0x07CA, 0x16C1, 0x1D09, 0x1D35, 0x1D4E, 0x1D62, 0x1D7B, 0x1D7C, 0x1D96, 0x1DA6, 0x1E2D, 0x1E2F, 0x1EC9, 0x1ECB, 0x1F30, 0x1F31, 0x1F32, 0x1F33, 0x1F34, 0x1F35, 0x1F36, 0x1F37, 0x1F76, 0x1F77, 0x1FD0, 0x1FD1, 0x1FD2, 0x1FD3, 0x1FD6, 0x1FD7, 0x2071, 0x2110, 0x2111, 0x2139, 0x2148, 0x2170, 0x217C, 0x2223, 0x2373, 0x23FD, 0x2C93, 0x2D4F, 0x4E28, 0x5DE5, 0xA4F2, 0xA647, 0xA749, 0xAB37, 0xAB38, 0xAB39, 0xAB75, 0xFE8D, 0xFFE8],
  &[0x0135, 0x01F0, 0x0237, 0x0249, 0x025F, 0x0279, 0x027A, 0x027B, 0x029D, 0x02B2, 0x03F3, 0x0458, 0x148D, 0x1D0A, 0x1D36, 0x2149, 0x2C7C, 0x52F9, 0xA4D9, 0xAB7B, 0xFF8C],
  &[0x0137, 0x0138, 0x0199, 0x01E9, 0x029E, 0x03BA, 0x03D7, 0x043A, 0x045C, 0x049B, 0x049D, 0x049F, 0x04A1, 0x04C4, 0x16D5, 0x1D0B, 0x1D37, 0x1D4F, 0x1D84, 0x1E31, 0x1E33, 0x1E35, 0x2096, 0x20AD, 0x2C6A, 0x2C95, 0x7247, 0xA4D7, 0xA741, 0xA743, 0xA745, 0xA7A3, 0xABB6],
  &[0x007C, 0x013A, 0x013C, 0x013E, 0x0140, 0x0142, 0x019A, 0x01AA, 0x01C0, 0x01C1, 0x0234, 0x0235, 0x026B, 0x026C, 0x026D, 0x0285, 0x029F, 0x02E1, 0x0661, 0x06F1, 0x1102, 0x14AA, 0x1D0C, 0x1D38, 0x1D85, 0x1DEC, 0x1E37, 0x1E39, 0x1E3B, 0x1E3D, 0x2097, 0x2112, 0x2113, 0x2142, 0x2143, 0x2C61, 0x2CD1, 0x3057, 0x3125, 0xA4E1, 0xA78E, 0xABAE, 0xFE8E],
  &[0x0271, 0x03BC, 0x03FB, 0x043C, 0x04CE, 0x15F0, 0x16D6, 0x1D0D, 0x1D1F, 0x1D39, 0x1D50, 0x1D5A, 0x1D6F, 0x1D86, 0x1E3F, 0x1E41, 0x1E43, 0x2098, 0x20A5, 0x2133, 0x217F, 0x2C99, 0x518A, 0x722A, 0xA4DF, 0xA773, 0xAB3A, 0xAB87],
  &[0x00F1, 0x0144, 0x0146, 0x0148, 0x0149, 0x014B, 0x014D, 0x014F, 0x0151, 0x019E, 0x01F9, 0x0235, 0x0272, 0x0273, 0x0274, 0x0377, 0x03AE, 0x03B7, 0x0438, 0x0439, 0x043B, 0x043F, 0x045D, 0x048B, 0x04C6, 0x04E3, 0x04E5, 0x0578, 0x057C, 0x144E, 0x1D0E, 0x1D3A, 0x1D3B, 0x1D70, 0x1D87, 0x1E45, 0x1E47, 0x1E49, 0x1E4B, 0x1F20, 0x1F21, 0x1F22, 0x1F23, 0x1F24, 0x1F25, 0x1F26, 0x1F27, 0x207F, 0x2099, 0x20A6, 0x2115, 0x2C9B, 0x51E0, 0xA4E0, 0xA774, 0xA791, 0xA7A5, 0xAB3B],
  &[0x00A4, 0x00B0, 0x00F0, 0x00F2, 0x00F3, 0x00F4, 0x00F5, 0x00F6, 0x00F8, 0x018D, 0x01A1, 0x01A3, 0x01D2, 0x01ED, 0x01FF, 0x020D, 0x020F, 0x0223, 0x022B, 0x022D, 0x022F, 0x0231, 0x0275, 0x0276, 0x0277, 0x0278, 0x0298, 0x03B8, 0x03BF, 0x03C3, 0x03CC, 0x03D9, 0x043E, 0x0473, 0x047B, 0x047D, 0x04E7, 0x04E9, 0x04EB, 0x0585, 0x05E1, 0x0647, 0x0665, 0x06BE, 0x06C1, 0x06D5, 0x06F5, 0x07C0, 0x0966, 0x09E6, 0x0A66, 0x0AE6, 0x0B20, 0x0B66, 0x0BE6, 0x0C02, 0x0C66, 0x0C82, 0x0CE6, 0x0D02, 0x0D20, 0x0D66, 0x0D82, 0x0E50, 0x0ED0, 0x101D, 0x1040, 0x10FF, 0x12D0, 0x1D0F, 0x1D10, 0x1D11, 0x1D12, 0x1D13, 0x1D16, 0x1D17, 0x1D3C, 0x1D52, 0x1D53, 0x1D54, 0x1D55, 0x1DED, 0x1DF3, 0x1E4D, 0x1E4F, 0x1E51, 0x1E53, 0x1ECD, 0x1ECF, 0x1ED1, 0x1ED3, 0x1ED5, 0x1ED7, 0x1ED9, 0x1EDB, 0x1EDD, 0x1EDF, 0x1EE1, 0x1EE3, 0x1F78, 0x1F79, 0x200E, 0x2092, 0x2134, 0x24C4, 0x24DE, 0x2C7A, 0x2C9F, 0x2D54, 0x3007, 0x3116, 0x56DE, 0xA4F3, 0xA74B, 0xA74D, 0xAB3D, 0xD40E, 0xD428, 0xD4AA, 0xD4C4, 0xD4DE, 0xD4F8, 0xD512, 0xD52C, 0xD546, 0xD560, 0xD57A, 0xD594, 0xD5E2, 0xD5FC, 0xD616, 0xD630, 0xD64A, 0xD664, 0xD67E, 0xD698, 0xF13E, 0xF17E, 0xFBA6, 0xFBA7, 0xFBA8, 0xFBA9, 0xFBAA, 0xFBAB, 0xFBAC, 0xFBAD, 0xFEE9, 0xFEEA, 0xFEEB, 0xFEEC, 0xFF2F, 0xFF4F],
  &[0x00B6, 0x00FE, 0x01A5, 0x01BF, 0x03C1, 0x03F8, 0x0440, 0x048F, 0x146D, 0x1D18, 0x1D3E, 0x1D56, 0x1D71, 0x1D7D, 0x1D88, 0x1E55, 0x1E57, 0x1FE4, 0x1FE5, 0x209A, 0x20B1, 0x2117, 0x2119, 0x2374, 0x2CA3, 0x5369, 0x5C38, 0xA4D1, 0xA751, 0xA753, 0xA755, 0xABB2],
  &[0x01EB, 0x0239, 0x024B, 0x02A0, 0x051B, 0x0563, 0x0566, 0x146B, 0x1D60, 0x1D90, 0x211A, 0x213A, 0x2D55, 0x7532, 0xA757, 0xA759],
  &[0x00AE, 0x0155, 0x0157, 0x0159, 0x0211, 0x0213, 0x024D, 0x027C, 0x027D, 0x027E, 0x027F, 0x0280, 0x0281, 0x02B3, 0x02B4, 0x02B5, 0x02B6, 0x0433, 0x044F, 0x0453, 0x0491, 0x04F7, 0x1587, 0x16B1, 0x1D19, 0x1D1A, 0x1D26, 0x1D3F, 0x1D63, 0x1D72, 0x1D73, 0x1D89, 0x1DCA, 0x1DE3, 0x1E59, 0x1E5B, 0x1E5D, 0x1E5F, 0x211B, 0x211C, 0x211D, 0x211E, 0x213E, 0x2C85, 0x5C3A, 0xA4E3, 0xA7A7, 0xAB47, 0xAB48, 0xAB49, 0xAB71, 0xAB81, 0xABA2],
  &[0x00A7, 0x015B, 0x015D, 0x015F, 0x0161, 0x01A8, 0x0219, 0x023F, 0x0282, 0x02E2, 0x03E9, 0x0455, 0x057F, 0x1515, 0x1D74, 0x1D8A, 0x1E61, 0x1E63, 0x1E65, 0x1E67, 0x1E69, 0x209B, 0x20B4, 0x4E02, 0x5DF1, 0xA4E2, 0xA731, 0xA7A9, 0xA7CA, 0xABA5, 0xABAA],
  &[0x0163, 0x0165, 0x0167, 0x01AB, 0x01AD, 0x021B, 0x0236, 0x0287, 0x0288, 0x0373, 0x03C4, 0x03EF, 0x0442, 0x04AD, 0x1D1B, 0x1D40, 0x1D57, 0x1D75, 0x1E6B, 0x1E6D, 0x1E6F, 0x1E71, 0x1E97, 0x2020, 0x209C, 0x20AE, 0x22A4, 0x27D9, 0x2C66, 0x2CA7, 0x3112, 0x535E, 0xA4D4, 0xAB72],
  &[0x00F9, 0x00FA, 0x00FB, 0x00FC, 0x0169, 0x016B, 0x016D, 0x016F, 0x0171, 0x0173, 0x01B0, 0x01D4, 0x01D6, 0x01D8, 0x01DA, 0x01DC, 0x0215, 0x0217, 0x0289, 0x028A, 0x028B, 0x03B0, 0x03C5, 0x03CB, 0x0446, 0x045F, 0x04B5, 0x057D, 0x1200, 0x144C, 0x1D1C, 0x1D1D, 0x1D1E, 0x1D41, 0x1D58, 0x1D59, 0x1D64, 0x1D7E, 0x1D7F, 0x1D99, 0x1DF0, 0x1DF4, 0x1E73, 0x1E75, 0x1E77, 0x1E79, 0x1E7B, 0x1EE5, 0x1EE7, 0x1EE9, 0x1EEB, 0x1EED, 0x1EEF, 0x1EF1, 0x1F7A, 0x1F7B, 0x1FE0, 0x1FE1, 0x1FE2, 0x1FE3, 0x222A, 0x22C3, 0x3129, 0x51F5, 0xA4F4, 0xA79F, 0xA7B9, 0xAB4E, 0xAB4F, 0xAB52],
  &[0x028C, 0x03BD, 0x0475, 0x0477, 0x05D8, 0x0667, 0x06F7, 0x142F, 0x1D20, 0x1D5B, 0x1D65, 0x1E7D, 0x1E7F, 0x2123, 0x2174, 0x2228, 0x22C1, 0x2C71, 0x2C74, 0x2D38, 0x30EC, 0xA4E6, 0xA6DF, 0xA75F, 0xABA9],
  &[0x0175, 0x026F, 0x0270, 0x028D, 0x02AC, 0x02B7, 0x0448, 0x0449, 0x0461, 0x047F, 0x051D, 0x0561, 0x15EF, 0x1ABF, 0x1D21, 0x1D42, 0x1E81, 0x1E83, 0x1E85, 0x1E87, 0x1E89, 0x1E98, 0x1F60, 0x1F61, 0x1F62, 0x1F63, 0x1F64, 0x1F65, 0x1F66, 0x1F67, 0x1F7C, 0x1F7D, 0x1FF2, 0x1FF3, 0x1FF4, 0x1FF6, 0x1FF7, 0x20A9, 0x2C73, 0x5C71, 0xA4EA, 0xAB83, 0xABA4, 0xFFE6],
  &[0x00D7, 0x02E3, 0x03C7, 0x03F0, 0x0436, 0x0445, 0x04B3, 0x04FD, 0x04FF, 0x1541, 0x157D, 0x166D, 0x166E, 0x16B7, 0x1D8D, 0x1E8B, 0x1E8D, 0x2093, 0x2135, 0x2179, 0x2573, 0x292B, 0x292C, 0x2A2F, 0x2CAD, 0x2D5D, 0x30E1, 0x4E42, 0xA4EB, 0xAB53, 0xAB56, 0xAB57, 0xAB58, 0xAB59],
  &[0x00A5, 0x00FD, 0x00FF, 0x0177, 0x019B, 0x01B4, 0x0233, 0x024F, 0x0263, 0x0264, 0x028E, 0x028F, 0x02B8, 0x03B3, 0x03BB, 0x03D2, 0x03D3, 0x03D4, 0x0443, 0x0447, 0x045E, 0x04AF, 0x04B1, 0x04B7, 0x04B9, 0x04CC, 0x04EF, 0x04F1, 0x04F3, 0x04F5, 0x10E7, 0x1D8C, 0x1E8F, 0x1E99, 0x1EF3, 0x1EF5, 0x1EF7, 0x1EF9, 0x1EFF, 0x213D, 0x2144, 0x2CA9, 0x3068, 0x311A, 0xA4EC, 0xAB5A, 0xAB79, 0xAB8D, 0xFFE5],
  &[0x017A, 0x017C, 0x017E, 0x01B6, 0x0225, 0x0240, 0x0290, 0x0291, 0x03B6, 0x1614, 0x1D22, 0x1D76, 0x1D8E, 0x1DBB, 0x1E91, 0x1E93, 0x1E95, 0x2124, 0x2128, 0x2C6C, 0x4E59, 0xA4DC, 0xAB93]
];

const ALPHABETICAL_2_ORDERS: [(u16, &'static [u16]); 2] = [
  (0xD835, &[0xDD1E, 0xDD04, 0xDD86, 0xDD6C, 0xDCEA, 0xDCD0, 0xDCB6,
             0xDC9C, 0xDD52, 0xDD38, 0xDC1A, 0xDC00, 0xDDEE, 0xDDD4,
             0xDE22, 0xDE08, 0xDE56, 0xDE3C, 0xDE8A, 0xDE70]),
  (0xD83C, &[0xDD30, 0xDD70])
];

const ALPHABETICAL_2: [(u8, &'static [(u16, u16)]); 23] = [
  (0x37, &[(0xD801, 0xDCD2), (0xD801, 0xDCFA)]),
  (0x38, &[(0xD800, 0xDF1A), (0xD835, 0xDFEA)]),
  (0x39, &[(0xD835, 0xDFEB)]),
  (0x61, &[(0xD835, 0xDEAB), (0xD835, 0xDEB2), (0xD835, 0xDEDB), (0xD835, 0xDEE5), (0xD835, 0xDEEC), (0xD835, 0xDF15), (0xD835, 0xDF59), (0xD835, 0xDF60), (0xD835, 0xDF89), (0xD835, 0xDF93), (0xD835, 0xDFC3)]),
  (0x62, &[(0xD835, 0xDEFD)]),
  (0x63, &[(0xD800, 0xDF02), (0xD801, 0xDC15), (0xD801, 0xDC3D), (0xD835, 0xDED3), (0xD835, 0xDF0D), (0xD835, 0xDF81), (0xD835, 0xDFBB)]),
  (0x65, &[(0xD835, 0xDEBA), (0xD835, 0xDEC6), (0xD835, 0xDEDC), (0xD835, 0xDEF4), (0xD835, 0xDF00), (0xD835, 0xDF09), (0xD835, 0xDF16), (0xD835, 0xDF1A), (0xD835, 0xDF68), (0xD835, 0xDF74), (0xD835, 0xDF8A), (0xD835, 0xDF8E), (0xD835, 0xDFA2), (0xD835, 0xDFAE), (0xD835, 0xDFC4), (0xD835, 0xDFC8)]),
  (0x66, &[(0xD835, 0xDFCB)]),
  (0x6C, &[(0xD801, 0xDC1B), (0xD801, 0xDC43)]),
  (0x69, &[(0xD835, 0xDECA), (0xD835, 0xDF04), (0xD800, 0xDF09), (0xD800, 0xDF20), (0xD835, 0xDF78), (0xD835, 0xDFB2)]),
  (0x6B, &[(0xD835, 0xDECB), (0xD835, 0xDF05), (0xD835, 0xDF79), (0xD835, 0xDFB3)]),
  (0x6E, &[(0xD835, 0xDEB7), (0xD835, 0xDEC8), (0xD835, 0xDEF1), (0xD835, 0xDF02), (0xD835, 0xDF65), (0xD835, 0xDF76), (0xD835, 0xDF9F), (0xD835, 0xDFB0)]),
  (0x6F, &[(0xD835, 0xDEAF), (0xD835, 0xDEC9), (0xD835, 0xDED0), (0xD835, 0xDEE9), (0xD835, 0xDF03), (0xD835, 0xDF5D), (0xD835, 0xDF67), (0xD835, 0xDF77), (0xD835, 0xDFB1), (0xD835, 0xDF97)]),
  (0x70, &[(0xD835, 0xDF0C)]),
  (0x71, &[(0xD835, 0xDED2), (0xD835, 0xDF80), (0xD835, 0xDFBA)]),
  (0x72, &[(0xD801, 0xDCB4), (0xD801, 0xDCDC), (0xD835, 0xDEAA), (0xD835, 0xDEE4), (0xD835, 0xDF58), (0xD835, 0xDF83), (0xD835, 0xDF92)]),
  (0x73, &[(0xD801, 0xDC20), (0xD801, 0xDC48), (0xD83D, 0xDF68), (0xD835, 0xDEFF)]),
  (0x74, &[(0xD800, 0xDF15), (0xD835, 0xDED5), (0xD835, 0xDF0F), (0xD835, 0xDFBD)]),
  (0x75, &[(0xD801, 0xDCCE), (0xD801, 0xDCF6), (0xD835, 0xDED6)]),
  (0x76, &[(0xD835, 0xDEC1), (0xD835, 0xDECE), (0xD835, 0xDEFB), (0xD835, 0xDF08), (0xD835, 0xDF6F), (0xD835, 0xDF7C), (0xD835, 0xDFA9), (0xD835, 0xDFB6)]),
  (0x77, &[(0xD800, 0xDF17), (0xD800, 0xDF22), (0xD835, 0xDEDA), (0xD835, 0xDF14), (0xD835, 0xDF88), (0xD835, 0xDFC2)]),
  (0x78, &[(0xD835, 0xDED8), (0xD835, 0xDEDE), (0xD835, 0xDF18), (0xD835, 0xDF86), (0xD835, 0xDF8C), (0xD835, 0xDFC0), (0xD835, 0xDFC6)]),
  (0x79, &[(0xD835, 0xDEBC), (0xD835, 0xDEC4), (0xD835, 0xDEF6), (0xD835, 0xDF06), (0xD835, 0xDF7A), (0xD835, 0xDFAC), (0xD835, 0xDFB4)]),
];

const ALPHABETICAL_PATTERN: [u16; 5] = [0xFF41, 0xFF21, 0x24D0, 0x24B6, 0x249C];

pub(crate) fn parse(input: &mut Vec<u16>) {
  let mut stop_iterating = false;
  
  for i in 0..input.len() {
    stop_iterating = false;
    
    for pat in ALPHABETICAL_PATTERN {
      if input[i] >= pat && input[i] <= (pat + 25) {
        input[i] = input[i] - pat + 97;
        stop_iterating = true;
        break;
      }
    }
    
    if !stop_iterating {
      for j in 0..ALPHABETICAL_1.len() {
        if ALPHABETICAL_1[j].contains(&input[i]) {
          input[i] = (j as u16) + 97;
          break;
        }
      }
    }
  }
  
  if input.len() == 0 {
    return;
  }
  
  let mut i = 0;
  
  while i < (input.len() - 1) {    
    for ch_order in ALPHABETICAL_2_ORDERS {
      if input[i] == ch_order.0 {
        for ch in ch_order.1 {
          if input[i + 1] >= *ch && input[i + 1] <= (*ch + 25) {
            stop_iterating = true;
            input[i + 1] = input[i + 1] - *ch + 0x61;
            input.remove(i);
            break;
          }
        }
        
        if stop_iterating {
          break;
        }
      }
    }
    
    if !stop_iterating {
      for (index, ch_list) in ALPHABETICAL_2 {
        for confusable in ch_list {
          if input[i] == confusable.0 && input[i + 1] == confusable.1 {
            input.remove(i + 1);
            input[i] = index as u16;
            stop_iterating = true;
            break;
          }
        }
        
        if stop_iterating {
          break;
        }
      }
    }
    
    stop_iterating = false;
    i += 1;
  }
}