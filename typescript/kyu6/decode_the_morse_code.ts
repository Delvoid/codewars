import { MORSE_CODE } from './preloaded';
export function decodeMorse(morseCode: string): string {
  let result = morseCode
    .trim()
    .split('   ')
    .map((word) =>
      word
        .split(' ')
        .map((letter) => MORSE_CODE[letter])
        .join('')
    )
    .join(' ');

  return result;
}
