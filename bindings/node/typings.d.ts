export class Match {
  readonly start: number
  readonly end: number
  toString(): string
}

export class CuredString {
  find(other: string): Match[]
  censor(other: string, character: string): void
  replace(other: string, withWhat: string): void  
  startsWith(other: string): boolean
  endsWith(other: string): boolean
  contains(other: string): boolean
  equals(other: string): boolean
  toString(): string
}

export type Options = {
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
}

export default function (input: string, options?: Options | number): CuredString

export function options(options?: Options): number
export function format(input: string): string
