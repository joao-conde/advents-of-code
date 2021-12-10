import { readFileSync } from "fs";

const CORRUPTION: Record<string, number> = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
};

const COMPLETION: Record<string, number> = {
    "(": 1,
    "[": 2,
    "{": 3,
    "<": 4
};

const process = (line: string): string | string[] => {
    const pairs: Record<string, string> = {
        "(": ")",
        "[": "]",
        "{": "}",
        "<": ">"
    };
    const open = Object.keys(pairs);
    const stack = [];
    for (const c of line.split("")) {
        if (open.includes(c)) {
            stack.push(c);
        } else {
            const top = stack.pop()!;
            if (pairs[top] !== c) return c;
        }
    }
    return stack;
};

const input = readFileSync("input/day10").toString().split("\n");
const processed = input.map(l => process(l));

const corrupt = processed.filter(illegal => typeof illegal === "string") as string[];
const p1 = corrupt.reduce((score, s) => score + CORRUPTION[s!], 0);
console.log("Part1:", p1);

const missing = processed.filter(missing => Array.isArray(missing)) as string[][];
const completionScores = missing
    .map(stack => stack!.reverse().reduce((score, c) => score * 5 + COMPLETION[c], 0))
    .sort((a, b) => a - b);
const p2 = completionScores[Math.floor(completionScores.length / 2)];
console.log("Part2:", p2);
