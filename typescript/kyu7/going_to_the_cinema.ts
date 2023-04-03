export function movie(card: number, ticket: number, perc: number): number {
  let systemA = 0;
  let systemB = card;
  let i = 0;
  while (Math.ceil(systemB) >= systemA) {
    systemA += ticket;
    systemB += ticket * Math.pow(perc, ++i);
  }
  return i;
}

console.log(movie(500, 15, 0.9));
