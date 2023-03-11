export function rps(p1: string, p2: string): string {
  if (p1 === p2) {
    return 'Draw!';
  }

  switch (p1) {
    case 'rock':
      return p2 === 'scissors' ? 'Player 1 won!' : 'Player 2 won!';
    case 'paper':
      return p2 === 'rock' ? 'Player 1 won!' : 'Player 2 won!';
    case 'scissors':
      return p2 === 'paper' ? 'Player 1 won!' : 'Player 2 won!';
    default:
      return 'Invalid input';
  }
}

const beats: { [index: string]: string } = {
  scissors: 'paper',
  paper: 'rock',
  rock: 'scissors',
};

export function rps2(p1: string, p2: string): string {
  if (p1 === p2) return 'Draw!';

  if (beats[p1] == p2) return 'Player 1 won!';
  return 'Player 2 won!';
}
