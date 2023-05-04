// Run: npx ts-node ./lineReader.ts

import fs from "fs";

// Print every line from file
fs.readFileSync("./lines")
  .toString()
  .split("\n")
  .forEach((line) => console.log(line));

console.log("---");

// Print every other line
fs.readFileSync("./lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .forEach((line) => console.log(line));

console.log("---");

// Every other line
// Skip two lines
// Print two lines
fs.readFileSync("./lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4)
  .forEach((line) => console.log(line));

console.log("---");
