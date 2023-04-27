export const calc = (str: string): number => {
    const ascii = str.split('').map((a) => a.charCodeAt(0)).join('').split('');
    const total1 = ascii.reduce((a, b) => a + parseInt(b), 0);
    const total2 = ascii.map((a)=> a === '7' ? '1' : a).reduce((a, b) => a + parseInt(b), 0);
    
    return total1 - total2;
}

console.log(calc('ABC'));