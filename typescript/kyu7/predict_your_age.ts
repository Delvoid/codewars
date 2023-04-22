export function predictAge(...ages: number[]): number {
    let sum: number = 0;
    for (let i: number = 0; i < ages.length; i++) {
        sum += ages[i] * ages[i];
    }
    return Math.floor(Math.sqrt(sum) / 2);
};
