export function sameCase(a: string, b: string): number {
  if (!isSingleLetter(a) || !isSingleLetter(b)) return -1;
  const aIsLower = a === a.toLowerCase();
  const bIsLower = b === b.toLowerCase();
  return aIsLower === bIsLower ? 1 : 0;
}

function isSingleLetter(input: string): boolean {
  const regex = /^[a-zA-Z]$/;
  return regex.test(input);
}

console.log(sameCase('a', 'a')); // 1
console.log(sameCase('a', 'A')); // 0
console.log(sameCase('/a', 'b')); // -1
console.log(sameCase('a', '.')); // -1
