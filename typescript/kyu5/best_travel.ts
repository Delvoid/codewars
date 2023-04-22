export function chooseBestSum(t: number, k: number, ls: number[]): number | null {
    function generateCombinations(arr: number[], k: number, startIndex: number, currentCombination: number[], allCombinations: number[][]): void {
        if (k === 0) {
            allCombinations.push([...currentCombination]);
            return;
        }

        for (let i = startIndex; i <= arr.length - k; i++) {
            currentCombination.push(arr[i]);
            generateCombinations(arr, k - 1, i + 1, currentCombination, allCombinations);
            currentCombination.pop();
        }
    }

    const allCombinations: number[][] = [];
    generateCombinations(ls, k, 0, [], allCombinations);

    let bestSum: number | null = null;
    for (const combination of allCombinations) {
        const sum = combination.reduce((acc, val) => acc + val, 0);
        if (sum <= t && (bestSum === null || sum > bestSum)) {
            bestSum = sum;
        }
    }

    return bestSum;
}


// Test cases
const ts = [50, 55, 56, 57, 58];
console.log(chooseBestSum(163, 3, ts)); // 163

const xs = [50];
console.log(chooseBestSum(163, 3, xs)); // null

const ys = [91, 74, 73, 85, 73, 81, 87];
console.log(chooseBestSum(230, 3, ys)); // 228