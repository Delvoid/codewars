export function vertMirror(strng: string) {
  return strng
    .split('\n')
    .map((s) => s.split('').reverse().join(''))
    .join('\n');
}
export function horMirror(strng: string) {
  return strng.split('\n').reverse().join('\n');
}
export function oper(fct: (s: string) => string, s: string) {
  return fct(s);
}

console.log(oper(vertMirror, 'hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu'));
