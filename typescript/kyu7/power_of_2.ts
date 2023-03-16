export function isPowerOfTwo(n: number): boolean {
  if (n <= 0) return false;
  if (n === 1) return true;

  let i = 1;
  while (i < n) {
    i *= 2;
  }

  return i === n;
}

export function isPowerOfTwo2(n: number): boolean {
  return Number.isInteger(Math.log2(n));
}
