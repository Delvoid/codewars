//return the total number of smiling faces in the array
export function countSmileys(arr: string[]) {
  if (arr.length === 0) return 0;
  return arr.filter((s) => /^[:;][-~]?[)D]$/.test(s)).length;
}
