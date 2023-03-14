export function revRot(s: string, sz: number): string {
  if (sz <= 0 || sz > s.length) return '';
  if (s.length === 0) return '';

  const chunks = s.match(new RegExp(`.{1,${sz}}`, 'g'));

  if (!chunks) return '';
  return chunks
    .filter((chunk) => chunk.length === sz)
    .map((chunk) => {
      const sum = chunk.split('').reduce((acc, curr) => acc + Number(curr) ** 3, 0);
      if (sum % 2 === 0) return chunk.split('').reverse().join('');
      return chunk.slice(1) + chunk[0];
    })
    .join('');
}

console.log(revRot('123456987654', 6)); // 234561876549
console.log(revRot('733049910872815764', 5));
