export function sumArray(array:number[] | null) : number {
    const sorted = array?.sort((a, b) => a - b) ?? [];
    return sorted.slice(1, sorted.length - 1).reduce((a, b) => a + b, 0);
}