export const likes = (names: string[]): string => {
  const [a, b, c] = names;
  switch (names.length) {
    case 0:
      return 'no one likes this';
    case 1:
      return `${a} likes this`;
    case 2:
      return `${a} and ${b} like this`;
    case 3:
      return `${a}, ${b} and ${c} like this`;
    default:
      return `${a}, ${b} and ${names.length - 2} others like this`;
  }
};
