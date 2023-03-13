export function order(words: string): string {
  const wordsArray = words.split(' ');
  const wordsArraySorted = wordsArray.sort((a, b) => {
    const aNumber = a.match(/\d/);
    const bNumber = b.match(/\d/);
    if (!aNumber || !bNumber) return 0;

    return Number(aNumber) - Number(bNumber);
  });
  return wordsArraySorted.join(' ');
}
