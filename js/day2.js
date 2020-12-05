import { readInput } from "./lib.js";

const input = readInput("day2-1");

const passwords = input.split("\n").map((l) => {
  const [range, letter, pass] = l.split(" ");
  const [min, max] = range.split("-").map(Number);

  return {
    min,
    max,
    letter: letter.slice(0, -1),
    pass,
  };
});

const validPasswords1 = passwords.filter(({ min, max, letter, pass }) => {
  const matches = pass.match(new RegExp(letter, "g"));
  if (!matches) return min === 0;

  return min <= matches.length && matches.length <= max;
});

console.log(`${validPasswords1.length} are valid`);

const validPasswords2 = passwords.filter(({ min, max, letter, pass }) => {
  // xor
  const aMatches = pass.charAt(min - 1) === letter;
  const bMatches = pass.charAt(max - 1) === letter;
  return aMatches !== bMatches;
});

console.log(`${validPasswords2.length} are valid`);
