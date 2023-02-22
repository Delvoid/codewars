export function thirt(n: number): number {
  const divisors = [1, 10, 9, 12, 3, 4];
  let sum = 0;

  while (true) {
    sum = Array.from(String(n), Number)
      .reverse()
      .reduce((acc, digit, index) => acc + digit * divisors[index % 6], 0);

    if (sum === n) {
      return sum;
    }

    n = sum;
  }
}
