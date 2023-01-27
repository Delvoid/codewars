export function pyramid(n: number) {
  const arr: number[][] = [];
  for (let i = 1; i <= n; i += 1) {
    const innerArr: number[] = [];
    for (let j = 0; j < i; j += 1) {
      innerArr.push(1);
    }
    arr.push(innerArr);
  }
  return arr;
}

export function pyramid2(n: number) {
  const arr: number[][] = [];
  for (let i = 0; i < n; i++) {
    arr.push(Array(i + 1).fill(1));
  }
  return arr;
}
