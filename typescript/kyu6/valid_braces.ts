export function validBraces(braces: string): boolean {
  const map: string[] = [];
  for (const c of braces) {
    console.log(c);
    if (c === '{' || c === '(' || c === '[') {
      map.push(c);
    } else {
      if (map.length === 0) return false;
      const lastValue = map[map.length - 1];
      if (
        (c === '}' && lastValue === '{') ||
        (c === ')' && lastValue === '(') ||
        (c === ']' && lastValue === '[')
      ) {
        map.pop();
      } else {
        break;
      }
    }
  }
  return map.length === 0;
}

console.log(validBraces('(){}[]'));
console.log(validBraces('([{}])'));
console.log(validBraces('(}'));
console.log(validBraces('[(])'));
console.log(validBraces('[({})](]'));
console.log(validBraces('(})'));
console.log(validBraces('}}]]))}])'));

export function validBraces2(braces: string): boolean {
  const stack: string[] = [];
  const start = ['(', '[', '{'];
  const end = [')', ']', '}'];

  for (let index = 0; index < braces.length; index++) {
    const char = braces[index];

    if (start.indexOf(char) > -1) {
      stack.push(char);
      continue;
    }

    if (stack.pop() === start[end.indexOf(char)]) {
      continue;
    }

    return false;
  }

  return stack.length === 0;
}
