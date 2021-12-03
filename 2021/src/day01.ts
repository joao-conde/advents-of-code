import { readFileSync } from "fs";

const increases = (xs: number[]): number => xs.slice(1).filter((x, i) => x > xs[i]).length;

const measures = readFileSync("input/day01")
    .toString()
    .split("\n")
    .map(x => parseInt(x));
console.log("Part1:", increases(measures));

const windowMeasures = measures
    .slice(0, -2)
    .map((_, i) => measures.slice(i, i + 3))
    .map(ms => ms.reduce((sum, m) => sum + m));
console.log("Part2:", increases(windowMeasures));
