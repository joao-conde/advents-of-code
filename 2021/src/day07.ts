import { readFileSync } from "fs";

import { sum } from "./utils";

const positions = readFileSync("input/day07")
    .toString()
    .split(",")
    .map(x => parseInt(x))
    .sort((a, b) => a - b)


let median: number | null = null;
if (positions.length % 2 === 0) {
    median = positions[positions.length / 2]
} else {
    const pos = Math.ceil(positions[positions.length / 2])
    median = (positions[pos] + positions[pos + 1]) / 2
}

const p1 = sum(positions.map(p => Math.abs(p - median!)))
console.log(p1)

const cost = (x: number): number => {
    return x === 1 ? 1 : cost(x - 1) + x;
}

const avg = Math.floor(sum(positions) / positions.length)
const p2 = sum(positions.map(p => cost(Math.abs(p - avg))))
console.log(p2)

