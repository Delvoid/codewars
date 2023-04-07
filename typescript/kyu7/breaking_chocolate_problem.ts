export function breakChocolate(n: number, m: number): number {
  if (n === 0 || m === 0) return 0;

  return n * m - 1;
}

export function breakChocolate2(n: number, m: number): number {
  return Math.max(n * m - 1, 0);
}
