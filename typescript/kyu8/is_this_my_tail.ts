export function correctTail(body: string, tail: string): boolean {
  const sub = body[body.length - 1];

  return sub === tail ? true : false;
}
