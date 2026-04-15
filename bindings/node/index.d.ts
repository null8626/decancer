export declare class CuredString {
  disableLeetspeak(switch: boolean): void
  disableAlphabeticalLeetspeak(switch: boolean): void
  find(other: string): Array<Match>
  findMultiple(other: Array<string>): Array<Match>
  censor(other: string, with: string): void
  censorMultiple(other: Array<string>, with: string): void
  replace(other: string, with: string): void
  replaceMultiple(other: Array<string>, with: string): void
  startsWith(other: string): boolean
  endsWith(other: string): boolean
  contains(other: string): boolean
  equals(other: string): boolean
  toString(): string
}

export declare class Match {
  get start(): number
  get end(): number
  toString(): string
}

export declare function cure(input: string, maybeOptions?: number | Options | undefined | null): CuredString

export declare function options(options?: Options | undefined | null): number

export interface Options {
  retainCapitalization?: boolean
  disableBidi?: boolean
  disableLeetspeak?: boolean
  disableAlphabeticalLeetspeak?: boolean
  retainDiacritics?: boolean
  retainGreek?: boolean
  retainCyrillic?: boolean
  retainHebrew?: boolean
  retainArabic?: boolean
  retainDevanagari?: boolean
  retainBengali?: boolean
  retainArmenian?: boolean
  retainGujarati?: boolean
  retainTamil?: boolean
  retainThai?: boolean
  retainLao?: boolean
  retainBurmese?: boolean
  retainKhmer?: boolean
  retainMongolian?: boolean
  retainChinese?: boolean
  retainJapanese?: boolean
  retainKorean?: boolean
  retainBraille?: boolean
  retainEmojis?: boolean
  retainTurkish?: boolean
  asciiOnly?: boolean
  alphanumericOnly?: boolean
  all?: boolean
  pureHomoglyph?: boolean
}
