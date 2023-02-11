export const high = (str: string): string => {
  const words = str.split(' ');
  const scores = words.map((word) => {
    return word.split('').reduce((acc, cur) => acc + cur.charCodeAt(0) - 96, 0);
  });
  return words[scores.indexOf(Math.max(...scores))];
};
