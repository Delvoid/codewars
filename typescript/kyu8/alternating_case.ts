export function toAlternatingCase(s: string): string {
  return s
    .split('')
    .map((x) => {
      return x === x.toUpperCase() ? x.toLowerCase() : x.toUpperCase();
    })
    .join('');
}
