export function maxRot(n:number):number {
    const arr = n.toString().split('');
    let max = Number(arr.join(''));
    for (let i = 0; i < arr.length - 1; i++) {
        arr.push(arr.splice(i, 1)[0]);
        max = Math.max(max, Number(arr.join('')));
    }
    return max;
}

console.log(maxRot(38458215));