import { readFileSync } from "fs";

const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x);

const increases = (xs: number[]): number => xs.slice(1).filter((x, i) => x > xs[i]).length;

const input = readFileSync("input/day01").toString();
const measures = input.split("\n").map(x => parseInt(x));
console.log("Part1: " + increases(measures));

const windowMeasures = measures.slice(0, -2).map((_, i) => sum(measures.slice(i, i + 3)));
console.log("Part2: " + increases(windowMeasures));
