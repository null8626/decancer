type DecancerFunction = (rawInput: string) => string;

export default interface Decancer extends DecancerFunction {
  contains: (decancered: string, noNoWord: string) => boolean;
}
