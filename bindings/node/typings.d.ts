export class Match {
  readonly start: number
  readonly end: number
  toString(): string
}

export class CuredString {
  find(other: string): Match[]
  findMultiple(other: string[]): Match[]
  censor(other: string, character: string): void
  censorMultiple(other: string[], character: string): void
  replace(other: string, withWhat: string): void
  replaceMultiple(other: string[], withWhat: string): void
  startsWith(other: string): boolean
  endsWith(other: string): boolean
  contains(other: string): boolean
  equals(other: string): boolean
  toString(): string
}

export type Options = {
  all?: boolean
  retainCapitalization?: boolean
  disableBidi?: boolean
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
  pureHomoglyph?: boolean
  asciiOnly?: boolean
  alphanumericOnly?: boolean
}

export default function (input: string, options?: Options | number): CuredString

export function options(options?: Options): number