export function get_age(age: string): number {
  return +age[0];
}

// Assume the test input string is always a valid string.
console.log(get_age('2 years old'));
