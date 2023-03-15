export const listSquared = (m: number, n: number): number[][] => {
  const result: number[][] = [];

  for (let i = m; i <= n; i++) {
    let sum = 0;

    for (let j = 1; j <= Math.sqrt(i); j++) {
      if (i % j === 0) {
        sum += j * j;

        if (i / j !== j) {
          sum += (i / j) * (i / j);
        }
      }
    }

    if (Number.isInteger(Math.sqrt(sum))) {
      result.push([i, sum]);
    }
  }
  return result;
};

console.log(listSquared(1, 250)); // [[1, 1], [42, 2500], [246, 84100]]
