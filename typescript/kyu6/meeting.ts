export function meeting(s: string): string {
  return s
    .split(';')
    .map((x) => `(${x.toUpperCase().split(':').reverse().join(', ')})`)
    .sort()
    .join('');
}

const s =
  'Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill';

console.log(meeting(s));
