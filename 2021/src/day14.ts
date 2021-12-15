import { range, readFileAsString } from "./utils";

const solve = (template: string[], rules: Record<string, string>, steps: number) => {
    const counts = template.reduce((counts: Record<string, number>, c) => {
        counts[c] = c in counts ? counts[c] + 1 : 1;
        return counts;
    }, {});

    let pairs: Record<string, number> = template
        .slice(0, -1)
        .map((_, i) => template.slice(i, i + 2).join(""))
        .reduce((pairs: Record<string, number>, p) => {
            pairs[p] = p in pairs ? pairs[p] + 1 : 1;
            return pairs;
        }, {});

    range(steps).forEach(_ => {
        pairs = Object.entries(pairs).reduce((pairs: Record<string, number>, [k, v]) => {
            const letter = rules[k];
            const p1 = k[0] + letter;
            const p2 = letter + k[1];
            counts[letter] = letter in counts ? counts[letter] + v : v;
            pairs[p1] = p1 in pairs ? pairs[p1] + v : v;
            pairs[p2] = p2 in pairs ? pairs[p2] + v : v;
            return pairs;
        }, {});
    });

    const max = Object.keys(counts).reduce((max, c) => Math.max(max, counts[c]), 0);
    const min = Object.keys(counts).reduce((min, c) => Math.min(min, counts[c]), max);
    return max - min;
};

const input = readFileAsString("input/day14").split("\n");
const template = input[0].split("");
const rules = input
    .slice(2)
    .map(r => r.split(" -> "))
    .reduce((rules: Record<string, string>, [input, ouput]: string[]) => {
        rules[input] = ouput;
        return rules;
    }, {});
console.log("Part1:", solve(template, rules, 10));
console.log("Part2:", solve(template, rules, 40));
