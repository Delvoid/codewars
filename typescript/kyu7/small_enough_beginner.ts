export function smallEnough(a: number[], limit: number): boolean {
  return a.every((x) => x <= limit);
}
