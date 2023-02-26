export function flattenAndSort(inputArray: number[][]): number[] {
  return inputArray.flat().sort((a, b) => a - b);
}
