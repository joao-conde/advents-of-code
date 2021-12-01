import { readFileSync } from "fs";

const input = readFileSync("input/day01").toString().split("\n");
const measures = input.map(x => parseInt(x));

const difs = measures.slice(1).map((m, i) => m - measures[i]);
const increases = difs.filter(d => d > 0).length;
console.log("Part1: " + increases);

const windows = measures.map((m, i) => (i + 3 <= measures.length ? measures.slice(i, i + 3) : []));
const windowMeasures = windows.map(w => w.reduce((acc, cur) => acc + cur, 0));
const windowDifs = windowMeasures.slice(1).map((m, i) => m - windowMeasures[i]);
const windowIncreases = windowDifs.filter(d => d > 0).length;
console.log("Part2: " + windowIncreases);
