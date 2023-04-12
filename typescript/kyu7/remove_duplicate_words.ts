export function removeDuplicateWords(s: string): string {
  const words = s.split(' ');
  const uniqueWords = new Set(words);
  return Array.from(uniqueWords).join(' ');
}
