import { mean, median, readFileAsString, sum } from "./utils";

const fuel = (positions: number[], position: number, cost: (p: number, pi: number) => number) =>
    sum(positions.map(p => cost(position, p)));

const positions = readFileAsString("input/day07")
    .split(",")
    .map(x => parseInt(x))
    .sort((a, b) => a - b);

const p1 = fuel(positions, median(positions, false), (p, pi) => Math.abs(p - pi));
console.log("Part1:", p1);

const cost = (x: number): number => (x <= 1 ? 1 : cost(x - 1) + x);
const p2 = fuel(positions, Math.floor(mean(positions)), (p, pi) => cost(Math.abs(p - pi)));
console.log("Part2:", p2);
