const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim();
const numbers = input.split("\n").map(Number).sort((a, b) => b - a);
const numbersSet = new Set(numbers)

let iterations = 0;

twoNumbers:
for (let n of numbers) {
  for (let i = numbers.length; i > 0; i--) {
    iterations++
    const n2 = numbers[i];
    const sum = n + n2;

    if (sum === 2020) {
      console.log(`${n} + ${n2} = 2020; ${n} * ${n2} = ${n * n2}`)
      console.log(`iterated ${iterations}`)
      break twoNumbers;
    }

    if (sum > 2020) {
      break;
    }
  }
}

iterations = 0;
for (let n of numbers) {
  iterations++;
  const n2 = 2020 - n;
  if (numbersSet.has(n2)) {
      console.log(`${n} + ${n2} = 2020; ${n} * ${n2} = ${n * n2}`)
      console.log(`iterated ${iterations}`)
      break;
  }
}

iterations = 0;
threeNumbers:
for (let n of numbers) {
  for (let i = numbers.length; i > 0; i--) {
    for (let j = numbers.length; j > i; j--) {
      iterations++
      const n2 = numbers[i];
      const n3 = numbers[j];
      const sum = n + n2 + n3;

      if (sum === 2020) {
        console.log(`${n} + ${n2} + ${n3} = 2020; ${n} * ${n2} * ${n3} = ${n * n2 * n3}`)
        console.log(`iterated ${iterations}`)
        break threeNumbers;
      }

      if (sum > 2020) {
        break;
      }
    }
  }
}

// Only works if there are no duplicates
iterations = 0;
for (let n of numbers) {
  for (let i = numbers.length; i > 0; i--) {
    iterations++
    const n2 = numbers[i];
    const n3 = 2020 - n - n2;

    if (n3 < 0) {
      break;
    }

    if (numbersSet.has(n3) && n2 !== n3) {
        console.log(`${n} + ${n2} + ${n3} = 2020; ${n} * ${n2} * ${n3} = ${n * n2 * n3}`)
        console.log(`iterated ${iterations}`)
        process.exit();
    }
  }
}