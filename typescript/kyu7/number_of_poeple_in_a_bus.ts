export function number(busStops: [number, number][]): number {
    return busStops.reduce((acc, [on,off]) => acc + on - off,0)
}