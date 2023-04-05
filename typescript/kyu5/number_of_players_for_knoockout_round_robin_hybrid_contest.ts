export function solve(n: number): number[] {
  const playerCounts: number[] = n === 1 ? [2] : [];
  const maxExp = Math.ceil(Math.log2(n));

  for (let a = 0; a <= maxExp; a++) {
    const p2 = 2 ** a - 3 / 2;
    const d = Math.sqrt(p2 ** 2 + 2 * n) - p2;

    // If 'd' is an odd number, calculate and add the player count to the result array
    if (d % 2 === 1) {
      playerCounts.push(2 ** a * d);
    }
  }

  return playerCounts;
}

console.log(solve(3)); // [3,4]
console.log(solve(12)); // [12]
