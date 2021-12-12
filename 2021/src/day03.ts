import { range, readFileAsString } from "./utils";

const powerRate = (report: string[], selector: (ones: number, zeros: number) => number): number => {
    const size = report[0].length;
    return range(size).reduce((rate, i) => {
        const ones = report.filter(num => num[i] === "1").length;
        const zeros = report.length - ones;
        return rate | (selector(ones, zeros) << (size - i - 1));
    }, 0);
};

const gasRate = (
    report: string[],
    filter: (ones: number, zeros: number, expected: string) => boolean
): number => {
    const size = report[0].length;
    for (let i = 0; i < size && report.length !== 1; i++) {
        const ones = report.filter(num => num[i] === "1").length;
        const zeros = report.length - ones;
        report = report.filter(num => filter(ones, zeros, num[i]));
    }
    return parseInt(report[0], 2);
};

const report = readFileAsString("input/day03").split("\n");

const gamma = powerRate(report, (ones, zeros) => (ones > zeros ? 1 : 0));
const epsilon = powerRate(report, (ones, zeros) => (ones < zeros ? 1 : 0));
console.log("Part1:", gamma * epsilon);

const oxygen = gasRate(report, (ones, zeros, expected) => (ones >= zeros ? "1" : "0") === expected);
const carbon = gasRate(report, (ones, zeros, expected) => (zeros <= ones ? "0" : "1") === expected);
console.log("Part2:", oxygen * carbon);
