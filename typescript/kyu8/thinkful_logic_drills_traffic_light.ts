export function updateLightUrg(current: string): string {
  return current === 'green' ? 'yellow' : current === 'yellow' ? 'red' : 'green';
}

export function updateLight(current: string): string {
  switch (current) {
    case 'green':
      return 'yellow';
    case 'yellow':
      return 'red';
    case 'red':
      return 'green';
    default:
      return 'green';
  }
}

const lights = {
  green: 'yellow',
  yellow: 'red',
  red: 'green',
};

export const updateLight3 = (current: string): string => {
  return lights[current];
};
