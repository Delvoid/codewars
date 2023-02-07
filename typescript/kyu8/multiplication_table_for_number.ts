export function multiTable(number: number): string {
  const table: string[] = [];
  for (let i = 1; i <= 10; i++) {
    table.push(`${i} * ${number} = ${i * number}`);
  }
  return table.join('\n');
}

console.log(multiTable(5));

export function multiTable2(number: number): string {
  return Array.from({ length: 10 }, (_, i) => `${i + 1} * ${number} = ${number * (i + 1)}`).join(
    '\n'
  );
}
