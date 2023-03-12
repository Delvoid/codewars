export function solve(s: string) {
  return s.split(/[aeiou]/).reduce((max, word) => {
    const sum = word.split('').reduce((total, letter) => total + letter.charCodeAt(0) - 96, 0);
    return sum > max ? sum : max;
  }, 0);
}

export function solve2(s: string) {
  // your code here
  return Math.max(
    ...s.split(/[aeiou]/).map((x) => [...x].reduce((a, b) => a + b.charCodeAt(0) - 96, 0))
  );
}

console.log(solve('zodiacs')); // 26
