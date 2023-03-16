export function between(a: number, b: number): number[] {
  if (a > b) return [];
  return Array.from({ length: b - a + 1 }, (_, i) => a + i);
}
