type Move = 'down' | 'up' | 'right' | 'left';

const fighters: string[][] = [
  ['Ryu', 'E.Honda', 'Blanka', 'Guile', 'Balrog', 'Vega'],
  ['Ken', 'Chun Li', 'Zangief', 'Dhalsim', 'Sagat', 'M.Bison'],
];

export function streetFighterSelection(fighters: string[][], position: number[], moves: Move[]) {
  const hoveredCharacters: string[] = [];
  let [row, col] = position;

  for (const move of moves) {
    switch (move) {
      case 'up':
        if (row > 0) row--;
        break;
      case 'down':
        if (row < fighters.length - 1) row++;
        break;
      case 'left':
        col = (col - 1 + fighters[0].length) % fighters[0].length;
        break;
      case 'right':
        col = (col + 1) % fighters[0].length;
        break;
    }
    hoveredCharacters.push(fighters[row][col]);
  }

  return hoveredCharacters;
}

console.log(streetFighterSelection(fighters, [0, 0], ['right', 'right']));
console.log(streetFighterSelection(fighters, [0, 0], ['right', 'down', 'right']));
console.log(streetFighterSelection(fighters, [0,0], ['up', 'left', 'right', 'left', 'left']));
console.log(streetFighterSelection(fighters, [0,0], ['up', 'left', 'right', 'left', 'left', 'down']));
