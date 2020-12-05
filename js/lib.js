import path from "path";
import fs from "fs";

export const readInput = (name) =>
  fs.readFileSync(path.resolve(`../inputs/${name}`), "utf-8").trim();
