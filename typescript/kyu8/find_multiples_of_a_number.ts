export function findMultiples(integer: number, limit: number): number[] {
  const result: number[] = [];
  for (let i = integer; i <= limit; i += integer) {
    result.push(i);
  }
  return result;
}

export function findMultiples2(integer: number, limit: number): number[] {
  return Array.from({ length: Math.floor(limit / integer) }, (_, i) => (i + 1) * integer);
}
