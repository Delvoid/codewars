export function abbrevName(name: string): string {
  return name
    .split(' ')
    .map((word) => word[0].toUpperCase())
    .join('.');
}
