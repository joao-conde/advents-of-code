import { readFileSync } from "fs";

const input = readFileSync("input/day03").toString();
const report = input.split("\n");

const powerRate = (report: string[], rateFn: (ones: number, zeros: number) => number): number => {
    const size = report[0].length;
    let rate = 0;
    for (let i = 0; i < size; i++) {
        const bits = report.map(num => num[i]);
        const ones = bits.filter(b => b === "1").length;
        const zeros = bits.length - ones;
        rate |= rateFn(ones, zeros) << (size - i - 1);
    }
    return rate;
};

const gamma = powerRate(report, (ones, zeros) => (ones > zeros ? 1 : 0));
const epsilon = powerRate(report, (ones, zeros) => (ones < zeros ? 1 : 0));
console.log("Part1:", gamma, epsilon, gamma * epsilon);

const gasRate = (
    report: string[],
    filterFn: (ones: number, zeros: number, expected: string) => boolean
): number => {
    const size = report[0].length;
    for (let i = 0; i < size && report.length !== 1; i++) {
        const bits = report.map(num => num[i]);
        const ones = bits.filter(b => b === "1").length;
        const zeros = bits.length - ones;
        report = report.filter(num => filterFn(ones, zeros, num[i]));
    }
    return parseInt(report[0], 2);
};

const oxygen = gasRate(report, (ones, zeros, expected) => (ones >= zeros ? "1" : "0") === expected);
const carbon = gasRate(report, (ones, zeros, expected) => (zeros <= ones ? "0" : "1") === expected);
console.log("Part2:", oxygen, carbon, oxygen * carbon);
