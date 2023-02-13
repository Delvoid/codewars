export function squareSum(numbers: number[]): number {
  return numbers.reduce((acc, cur) => acc + cur * cur, 0);
}

console.log(squareSum([1, 2])); // 5
// 2 * 2 + 1 * 1 = 5

console.log(squareSum([0, 3, 4, 5])); // 50
// 3 * 3 + 4 * 4 + 5 * 5 = 50
