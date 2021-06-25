/*
Write a function named sumDigits which takes a number as input and returns the sum of the absolute value of each of the number's decimal digits. For example:

  sumDigits(10);  // Returns 1
  sumDigits(99);  // Returns 18
  sumDigits(-32); // Returns 5
Let's assume that all numbers in the input will be integer values.
*/
const sumDigits = (n) => {
  let num = Math.abs(n).toString()
  let sum = 0
  for (let i = 0; i < num.length; i++) {
    let number = parseInt(num[i])
    sum += Math.abs(number)
  }
  return sum
}