export const mxdiflg = (a1: string[], a2: string[]): number => {
  if (a1.length === 0 || a2.length === 0) {
    return -1;
  }
  const a1Max = Math.max(...a1.map((x) => x.length));
  const a1Min = Math.min(...a1.map((x) => x.length));
  const a2Max = Math.max(...a2.map((x) => x.length));
  const a2Min = Math.min(...a2.map((x) => x.length));
  return Math.max(a1Max - a2Min, a2Max - a1Min);
};
