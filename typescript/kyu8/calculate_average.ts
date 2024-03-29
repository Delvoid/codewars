export function findAverage(array: number[]): number {
  if (array.length === 0) return 0;
  return array.reduce((a, b) => a + b, 0) / array.length;
}
