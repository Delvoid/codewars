export function strCount(str: string, letter: string): number {
  return str.split(letter).length - 1;
}

export function strCount2(str: string, letter: string): number {
  return str.split("").filter((value) => value == letter).length;
}
