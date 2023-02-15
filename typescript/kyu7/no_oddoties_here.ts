export function noOdds(values: number[]): number[] {
  return values.filter((values) => values % 2 === 0);
}
