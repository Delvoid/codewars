export const strongNumber = (num: number): string => {
    let result = 0;
    const arr = num.toString().split('');
    for (let i = 0; i < arr.length; i++) {
        result += factorial(Number(arr[i]));
    }
    return result === num ? 'STRONG!!!!' : 'Not Strong !!';
};

const factorial = (num: number): number => {
    if (num === 0) return 1;
    return num * factorial(num - 1);
}