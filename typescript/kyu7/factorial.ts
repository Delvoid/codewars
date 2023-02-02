export function factorial(n: number) {
  if (n === 0 || n === 1) return 1;

  for (let i = n - 1; i >= 1; i--) {
    n *= i;
  }
  return n;
}

console.log(factorial(4));

// 4 x 3 x 2 x 1

export function factorialRec(n: number): number {
  if (n < 0) return -1;
  if (n == 0) return 1;
  return n * factorialRec(n - 1);
}

console.log(factorialRec(4));
