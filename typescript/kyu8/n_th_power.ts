export function index(array: number[], n: number): number {
  if (n >= array.length) return -1;
  return Math.pow(array[n], n);
}

// array = [1, 2, 3, 4] and N = 2, then the result is 3^2 == 9;
console.log(index([1, 2, 3, 4], 2));

export const index2 = (array: number[], n: number): number => array[n] ** n || -1;
