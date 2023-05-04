// Run: npx ts-node ./lineReader.ts

import fs from "fs";

// Print every line from file
fs.readFileSync("./lines")
  .toString()
  .split("\n")
  .forEach((line) => console.log(line));

// Print every other line
fs.readFileSync("./lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .forEach((line) => console.log(line));
