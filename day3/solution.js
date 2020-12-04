const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim();

const map = input.split("\n").map((row) => row.split(""));

const countTrees = (right, down) => {
  let x = 0;
  let y = 0;
  let width = map[0].length;
  let treesEncountered = 0;
  while (y < map.length) {
    if (map[y][x] === "#") {
      treesEncountered++;
    }
    x = (x + right) % width;
    y += down;
  }

  console.log(`for Right ${right}, down ${down} -- found ${treesEncountered}`);

  return treesEncountered;
};

countTrees(3, 1);
console.log("--");

const product = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
].reduce((total, [right, down]) => total * countTrees(right, down), 1);

console.log(`product of trees encountered ${product} trees`);
