export function capitalize(s: string) {
  return [0, 1].map((i) =>
    s
      .split('')
      .map((c, j) => (j % 2 === i ? c.toUpperCase() : c))
      .join('')
  );
}
console.log(capitalize('abcdef')); // [ 'AbCdEf', 'aBcDeF' ]
