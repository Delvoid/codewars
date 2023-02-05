export function reverseWords(str: string): string {
  return str
    .split(' ')
    .map((w) => w.split('').reverse().join(''))
    .join(' ');
}

console.log(reverseWords('a b c d'));
console.log(reverseWords('The quick brown fox jumps over the lazy dog.'));
