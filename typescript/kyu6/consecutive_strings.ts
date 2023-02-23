export function longestConsec(strarr: string[], k: number): string {
  if (strarr.length === 0 || k > strarr.length || k <= 0) return '';

  let longest = '';
  for (let i = 0; i < strarr.length - k + 1; i++) {
    let current = '';
    for (let j = i; j < i + k; j++) {
      current += strarr[j];
    }
    if (current.length > longest.length) {
      longest = current;
    }
  }
  return longest;
}

export function longestConsec2(strarr: string[], k: number): string {
  let max = '';
  const n = strarr.length;
  if (n === 0 || k > n || k <= 0) return max;

  for (let i = 0; i <= n - k; i++) {
    const newStr = strarr.slice(i, i + k).join('');
    max = max.length >= newStr.length ? max : newStr;
  }
  return max;
}
