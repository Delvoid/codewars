export function seven(m: number): number[] {
  let steps = 0;
  let n = m;
  while (n > 99) {
    n = Math.floor(n / 10) - 2 * (n % 10);
    steps++;
  }
  return [n, steps];
}
