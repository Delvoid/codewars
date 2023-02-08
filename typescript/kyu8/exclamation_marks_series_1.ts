// Exclamation marks series #1: Remove an exclamation mark from the end of string
export function remove(s: string): string {
  return s.slice(-1) === '!' ? s.slice(0, -1) : s;
}
