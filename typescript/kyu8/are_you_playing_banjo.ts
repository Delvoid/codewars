export function areYouPlayingBanjo(name: string): string {
  const firstChar = name[0].toLowerCase();
  return `${name} ${firstChar === 'r' ? 'plays banjo' : 'does not play banjo'}`;
}
