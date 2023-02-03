export function checkExam(array1: string[], array2: string[]): number {
  let score = 0;
  array2.forEach((n, i) => {
    console.log({ n, corr: array1[i] });
    if (n === array1[i]) {
      score += 4;
    } else if (n === '') {
      score -= 0;
    } else {
      score -= 1;
    }
    console.log(score);
  });

  return Math.max(score, 0);
}

console.log(checkExam(['b', 'c', 'b', 'a'], ['', 'a', 'a', 'c']));

// codewars give a type error for this one
export const checkExamMap = (correctAnswers: string[], studentAnswers: string[]): number => {
  const score = studentAnswers
    .map((answer, index): number => {
      if (answer === '') return 0;
      if (answer === correctAnswers[index]) return 4;
      return -1;
    })
    .reduce((acc, i) => acc + i, 0);
  return Math.max(score, 0);
};
export function checkExamReduce(array1: string[], array2: string[]): number {
  const score = array2.reduce((acc, it, i) => {
    if (!it) return acc;
    if (it === array1[i]) return acc + 4;

    return acc - 1;
  }, 0);

  return Math.max(score, 0);
}
