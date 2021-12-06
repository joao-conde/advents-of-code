import { readFileSync } from "fs";

import { range, sum } from "./utils";

const countFishes = (fishes: number[], days: number): number => {
    const states = range(10);
    const counts = states.map(s => fishes.filter(f => f === s).length);
    range(days).forEach(() => {
        counts[9] += counts[0];
        counts[7] += counts[0];
        counts[0] = 0;
        states.forEach(s => (counts[s] = counts[s + 1] || 0));
    });
    return sum(counts);
};

const fishes = readFileSync("input/day06")
    .toString()
    .split(",")
    .map(x => parseInt(x));
console.log("Part1:", countFishes(fishes, 80));
console.log("Part2:", countFishes(fishes, 256));
