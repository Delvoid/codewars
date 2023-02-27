export function isSortedAndHow(array: number[]): string {
  if (array.every((v, i, a) => !i || a[i - 1] <= v)) return 'yes, ascending';
  if (array.every((v, i, a) => !i || a[i - 1] >= v)) return 'yes, descending';
  return 'no';
}
