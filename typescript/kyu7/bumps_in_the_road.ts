export function bump(x: string): string {
  return x.split('').filter((c) => c === 'n').length > 15 ? 'Car Dead' : 'Woohoo!';
}
