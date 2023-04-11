export function cubeChecker(volume: number, side: number): boolean {
  if (volume <= 0 || side <= 0) {
    return false;
  }
  const calculatedSide = Math.cbrt(volume);
  return calculatedSide === side;
}
