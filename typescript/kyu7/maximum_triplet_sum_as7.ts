export function maxTriSum(nums: number[]): number {
  return [...new Set(nums)]
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((a, b) => a + b);
}
