import { readFileAsString, sum, windows } from "./utils";

const increases = (xs: number[]): number => xs.slice(1).filter((x, i) => x > xs[i]).length;

const measures = readFileAsString("input/day01")
    .split("\n")
    .map(x => parseInt(x));
console.log("Part1:", increases(measures));

const windowMeasures = windows(measures, 3).map(ms => sum(ms));
console.log("Part2:", increases(windowMeasures));
