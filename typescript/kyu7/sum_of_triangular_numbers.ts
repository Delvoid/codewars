export function sumTriangularNumbers(n: number): number {
  if (n < 1) return 0;

  let sum = 0;
  for (let i = 1; i <= n; i++) {
    sum += (i * (i + 1)) / 2;
  }
  return sum;
}

sumTriangularNumbers(6);
