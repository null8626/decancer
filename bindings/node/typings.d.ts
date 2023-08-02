export class CuredString {
  startsWith(other: string): boolean
  endsWith(other: string): boolean
  contains(other: string): boolean
  equals(other: string): boolean
  toString(): string
}

export default function (input: string): CuredString
