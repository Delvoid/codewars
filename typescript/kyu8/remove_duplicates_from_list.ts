export const distinct = (a: number[]): number[] => {
  // a.filter((e, i) => a.lastIndexOf(e) === i);
  return [...new Set(a)];
};
