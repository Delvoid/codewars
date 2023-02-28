export function enough(cap: number, on: number, wait: number): number {
  return Math.max(0, on + wait - cap);
}
