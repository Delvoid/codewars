export function doubleChar(str: string): string {
  return str
    .split('')
    .map((c) => c + c)
    .join('');
}
