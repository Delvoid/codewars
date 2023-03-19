export const race = (v1: number, v2: number, g: number) => {
  if (v1 >= v2) return null;
  const timeInSeconds = Math.floor((g / (v2 - v1)) * 3600);

  // Convert seconds to hours, minutes and seconds
  const hours = Math.floor(timeInSeconds / 3600);
  const minutes = Math.floor((timeInSeconds % 3600) / 60);
  const seconds = Math.floor(timeInSeconds % 60);

  return [hours, minutes, seconds];
};
