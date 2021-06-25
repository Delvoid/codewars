/*
Your task, is to create NxN multiplication table, of size provided in parameter.

for example, when given size is 3:

1 2 3
2 4 6
3 6 9
for given example, the return value should be: [[1,2,3],[2,4,6],[3,6,9]]
*/
multiplicationTable = function (size) {
  const arr = []
  for (let i = 1; i <= size; i++) {
    const temp = []
    for (let y = 1; y <= size; y++) {
      temp.push(y * i)
    }

    arr.push(temp)
  }
  console.log(arr)
  return arr
}