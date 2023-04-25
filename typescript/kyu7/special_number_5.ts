export function specialNumber(n: number) {
    const specialNumbers = ["0", "1", "2", "3", "4", "5"];
    const includesNumber = n.toString().split("").every((digit) => specialNumbers.includes(digit))
    return includesNumber ? "Special!!" : "NOT!!";
}

console.log(specialNumber(2));
console.log(specialNumber(3));
console.log(specialNumber(26));
console.log(specialNumber(25432));



export function specialNumber2(n: number) {
  return /[^0-5]/.test(n.toString()) ? 'NOT!!' : 'Special!!';
}