export function evaporator(content: number, evapPerDay: number, threshold: number): number {
  let days: number = 0;
  let currentContent: number = content;
  while (currentContent > (content * threshold) / 100) {
    currentContent -= (currentContent * evapPerDay) / 100;
    days++;
  }
  return days;
}
