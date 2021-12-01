import { readFileSync } from "fs";

const sum = (array: number[]): number => array.reduce((acc, cur) => acc + cur, 0);

const countIncreases = (measures: number[]): number =>
    measures
        .slice(1)
        .map((m, i) => m - measures[i])
        .filter(d => d > 0).length;

const input = readFileSync("input/day01").toString();

const measures = input.split("\n").map(x => parseInt(x));
console.log("Part1: " + countIncreases(measures));

const windowMeasures = measures.map((_, i) =>
    i + 3 <= measures.length ? sum(measures.slice(i, i + 3)) : 0
);
console.log("Part2: " + countIncreases(windowMeasures));
