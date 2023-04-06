export const isPangram = (phrase: string): boolean => {
  const alphabet = 'abcdefghijklmnopqrstuvwxyz';
  const phraseLetters = phrase.toLowerCase().replace(/[^a-z]/g, '');
  const phraseLettersSet = new Set(phraseLetters);
  return alphabet.split('').every((letter) => phraseLettersSet.has(letter));
};

export const isPangram2 = (phrase: string): boolean =>
  new Set(phrase.toLowerCase().match(/[a-z]/g)).size === 26;
