export function betterThanAverage(classPoints: number[], yourPoints: number): boolean {
  const avg = classPoints.reduce((a, b) => a + b, 0) / classPoints.length;
  return yourPoints > avg;
}
