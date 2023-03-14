export function minSum(arr: number[]): number {
  return (
    arr
      .sort((a, b) => a - b)
      .reduce((acc, curr, i, arr) => acc + curr * arr[arr.length - 1 - i], 0) / 2
  );
}

console.log(minSum([12, 6, 10, 26, 3, 24])); // 342
