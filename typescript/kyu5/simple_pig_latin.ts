export const pigIt = (a: string): string => {
  return a
    .split(' ')
    .map((word) => {
      const firstLetter = word.match(/^[a-zA-Z]/);
      return firstLetter ? word.slice(1) + firstLetter[0] + 'ay' : word;
    })
    .join(' ');
};

console.log(pigIt('Pig latin is cool')); // igPay atinlay siay oolcay
console.log(pigIt('Hello world !')); // elloHay orldway !
