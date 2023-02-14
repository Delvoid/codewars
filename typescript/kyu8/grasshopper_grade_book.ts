export function getGrade(a: number, b: number, c: number): string {
  const average = (a + b + c) / 3;
  switch (true) {
    case average >= 90:
      return 'A';
    case average >= 80:
      return 'B';
    case average >= 70:
      return 'C';
    case average >= 60:
      return 'D';
    default:
      return 'F';
  }
}

export function getGrade2(a: number, b: number, c: number): string {
  const mean = (a + b + c) / 3;
  if (mean >= 90) return 'A';
  if (mean >= 80) return 'B';
  if (mean >= 70) return 'C';
  if (mean >= 60) return 'D';
  return 'F';
}
