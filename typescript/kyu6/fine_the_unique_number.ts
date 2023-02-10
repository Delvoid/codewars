export function findUniq(arr: number[]): number {
  return arr.find((n) => arr.indexOf(n) === arr.lastIndexOf(n)) || 0;
}

console.log(findUniq([1, 1, 1, 2, 1, 1]) === 2);
console.log(findUniq([0, 0, 0.55, 0, 0]) === 0.55);

export function findUniq2(arr: Array<number>): number {
  arr = arr.sort();
  return arr[0] == arr[1] ? arr[arr.length - 1] : arr[0];
}
