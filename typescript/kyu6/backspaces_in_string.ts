export function cleanString(s: string): string {
  const arr = s.split('');
  const result: string[] = [];

  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === '#') {
      result.pop();
    } else {
      result.push(arr[i]);
    }
  }

  return result.join('');
}
