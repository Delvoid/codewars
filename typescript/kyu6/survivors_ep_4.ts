export function survivors(listOfMomentum: number[], listOfPowerups: number[][]): number[] {
  const results: number[] = [];

  for (let i = 0; i < listOfMomentum.length; i++) {
    let momentum = listOfMomentum[i];

    if (momentum <= 0) continue;

    if (listOfPowerups[i].length === 0) {
      results.push(i);
      continue;
    }

    for (let j of listOfPowerups[i]) {
      //cost to get in
      momentum = momentum - 1 + j;
      if (momentum <= 0) break;
    }

    if (momentum > 0) {
      results.push(i);
    }
  }
  return results;
}
console.log(survivors([9, 9, 8, 2, 6], [[], [0, 0, 0, 5, 1], [], [0, 0, 0, 0], [0]])); //[0, 1, 2, 4]
console.log(survivors([2], [[0, 0, 0, 0]])); //[ ]
console.log(survivors([2], [[0]]));
console.log(survivors([5], [[0, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0, 0]]));
