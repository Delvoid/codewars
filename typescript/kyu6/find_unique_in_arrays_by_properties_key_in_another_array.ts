type AnyObject = { [key: string]: any };

export function unique(objs: AnyObject[], keys: string[]): AnyObject[] {
  const uniqueMap = new Map<string, AnyObject>();

  for (const obj of objs) {
    const uniqueKeyValues: string[] = [];

    for (const key of keys) {
      uniqueKeyValues.push(obj[key] === undefined ? 'not_defined' : obj[key]);
    }

    const uniqueKey = uniqueKeyValues.join('|');

    if (!uniqueMap.has(uniqueKey)) {
      uniqueMap.set(uniqueKey, obj);
    }
  }

  return Array.from(uniqueMap.values());
}
export function uniquev3<T extends AnyObject>(objs: T[], keys: string[]): T[] {
  const uniqueSet = new Set<string>();
  const uniqueObjs: T[] = [];

  for (const obj of objs) {
    const uniqueKeyValues = keys.map((key) => obj[key] ?? 'not_defined');
    const uniqueKey = uniqueKeyValues.join('|');

    if (!uniqueSet.has(uniqueKey)) {
      uniqueSet.add(uniqueKey);
      uniqueObjs.push(obj);
    }
  }

  return uniqueObjs;
}
const objs = [
  { x: 1, y: 1 },
  { x: 2, y: 2 },
  { x: 1, z: 1 },
  { x: 1, y: 1, z: 3 },
];
const keys = ['x', 'y'];

console.log(unique(objs, keys));

export function unique2<T extends AnyObject>(arr: T[], keys: string[]): T[] {
  return arr.reduce((acc: T[], cur: T) => {
    const exist = acc.find((v: T) => keys.every((k) => cur[k] === v[k]));
    if (!exist) {
      acc.push(cur);
    }
    return acc;
  }, []);
}
