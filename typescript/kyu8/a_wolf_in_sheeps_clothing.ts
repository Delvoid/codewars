export function warnTheSheep(queue: string[]): string {
  const wolfIndex = queue.reverse().indexOf('wolf');
  return wolfIndex === 0
    ? 'Pls go away and stop eating my sheep'
    : `Oi! Sheep number ${wolfIndex}! You are about to be eaten by a wolf!`;
}
