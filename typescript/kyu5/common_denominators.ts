export const convertFrac = (lst: [number, number][]): string => {
  const gcd = (a: number, b: number): number => {
    if (b === 0) {
      return a;
    }
    return gcd(b, a % b);
  };

  // Simplify each fraction before finding the LCM
  const simplifiedFractions = lst.map(([numerator, denominator]) => {
    const commonFactor = gcd(numerator, denominator);
    return [numerator / commonFactor, denominator / commonFactor];
  });

  // Find the LCM of the denominators of the simplified fractions
  const lcm = simplifiedFractions.reduce((a, b) => {
    return (a * b[1]) / gcd(a, b[1]);
  }, 1);

  // Convert each fraction to an equivalent fraction with the LCM as the denominator
  const result = simplifiedFractions.map(([numerator, denominator]) => [
    (numerator * lcm) / denominator,
    lcm,
  ]);

  // Convert the result to the desired output format
  return result.map(([numerator, denominator]) => `(${numerator},${denominator})`).join('');
};

console.log(
  convertFrac([
    [1, 2],
    [4, 5],
    [3, 4],
    [6, 9],
    [7, 10],
  ])
);
