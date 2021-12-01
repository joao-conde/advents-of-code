import { readFileSync } from "fs";

const sum = (array: number[]): number => array.reduce((acc, cur) => acc + cur);

const countIncreases = (measures: number[]): number =>
    measures.slice(1).filter((m, i) => m > measures[i]).length;

const input = readFileSync("input/day01").toString();

const measures = input.split("\n").map(x => parseInt(x));
console.log("Part1: " + countIncreases(measures));

const windowMeasures = measures.map((_, i) =>
    i + 3 <= measures.length ? sum(measures.slice(i, i + 3)) : 0
);
console.log("Part2: " + countIncreases(windowMeasures));
