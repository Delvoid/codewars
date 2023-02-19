// []-- > []
// [("a", "b", "c")]-- > ["1: a", "2: b", "3: c"];

export function number(array: string[]): string[] {
  return array.map((item, idx) => `${idx + 1}: ${item}`);
}
