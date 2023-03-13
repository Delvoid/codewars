export function sumOfDifferences(arr: number[]): number {
  if (arr.length < 2) return 0;
  return Math.max(...arr) - Math.min(...arr);
}
