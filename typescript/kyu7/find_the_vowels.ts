export function vowelIndices(word: string): number[] {
  const vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
  return word
    .split('')
    .map((c, i) => (vowels.includes(c.toLowerCase()) ? i + 1 : 0))
    .filter((i) => i > 0);
}
