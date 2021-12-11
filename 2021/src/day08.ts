import { readFileSync } from "fs";

import { sum } from "./utils";

const decodeOutputs = (mapper: string[][], outputs: string[][]): number => {
    const code = outputs.reduce((code, chars) => code + decodeOutput(mapper, chars), "");
    return parseInt(code);
};

const decodeOutput = (mapper: string[][], output: string[]): number =>
    mapper.findIndex(k => k.every(x => output.includes(x) && output.every(x => k.includes(x))));

/**
 * Finds the mapping between signal wires and segments.
 * Side names are:
 *     side0
 *  side1 side2
 *     side3
 *  side4 side5
 *     side6
 */
const buildMapper = (patterns: string[][]): string[][] => {
    const one = patterns.find(p => p.length === 2)!;
    const four = patterns.find(p => p.length === 4)!;
    const seven = patterns.find(p => p.length === 3)!;
    const eight = patterns.find(p => p.length === 7)!;
    const sides234 = patterns
        .filter(p => p.length === 6)
        .flatMap(p => eight.filter(pi => !p.includes(pi)));
    const side4 = sides234.find(s => !four.includes(s))!;
    const nine = eight.filter(i => i !== side4);
    const side2 = sides234.find(s => one.includes(s));
    const six = eight.filter(i => i !== side2);
    const side3 = sides234.find(s => ![side2, side4].includes(s));
    const zero = eight.filter(i => i !== side3);
    const five = eight.filter(i => ![side2, side4].includes(i));
    const side5 = one.find(i => six.includes(i));
    const side1 = four.find(i => ![side2, side3, side5].includes(i));
    const three = eight.filter(i => ![side1, side4].includes(i));
    const two = three.concat([side4]).filter(i => i !== side5);
    return [zero, one, two, three, four, five, six, seven, eight, nine];
};

const parse = (line: string): [string[][], string[][]] => {
    const [patterns, outputs] = line.split("|");
    return [
        patterns
            .trim()
            .split(" ")
            .map(p => p.split("")),
        outputs
            .trim()
            .split(" ")
            .map(o => o.split(""))
    ];
};

const input = readFileSync("input/day08").toString().split("\n");

const p1 = sum(
    input
        .map(l => parse(l))
        .map(([_, outputs]) => outputs.filter(o => [2, 3, 4, 7].includes(o.length)).length)
);
console.log("Part1:", p1);

const p2 = sum(
    input
        .map(l => parse(l))
        .map(([patterns, outputs]) => decodeOutputs(buildMapper(patterns), outputs))
);
console.log("Part2:", p2);
