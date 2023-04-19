export function adjacentElementsProduct(arr: number[]): number {
  let max: number = arr[0] * arr[1];
  for (let i = 1; i < arr.length - 1; i++) {
    max = Math.max(max, arr[i] * arr[i + 1]);
  }
  return max;
}
