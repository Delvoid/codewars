export function generateShape(int: number): string {
  let result: string = '';
  for (let i = 0; i < int; i++) {
    for (let j = 0; j < int; j++) {
      result += '+';
    }
    if (i < int - 1) {
      result += '\n';
    }
  }
  return result;
}

console.log(generateShape(3));
console.log(generateShape(8));

export function generateShape2(int: number): string {
  const array: string[] = [];
  for (let i = 0; i < int; i++) {
    array.push('+'.repeat(int));
  }

  return array.join('\n');
}

export function generateShape3(int: number): string {
  return Array(int)
    .fill(Array(int + 1).join('+'))
    .join('\n');
}
