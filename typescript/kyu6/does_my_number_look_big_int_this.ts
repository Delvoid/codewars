// 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
export function narcissistic(value: number): boolean {
  const arr = value.toString().split('');
  const length = arr.length;
  const count = arr.map((n) => Math.pow(+n, length)).reduce((a, b) => a + b);
  return count === value;
}

console.log(narcissistic(153));
