import { range, readFileAsString } from "./utils";

const input = readFileAsString("input/day14").split("\n");
const template = input[0].split("");
const rules = input
    .slice(2)
    .map(r => r.split(" -> "))
    .reduce((rules: Record<string, string>, [input, ouput]: string[]) => {
        rules[input] = ouput;
        return rules;
    }, {});

const counts = template.reduce((counts: Record<string, number>, c) => {
    if (!(c in counts)) counts[c] = 0;
    counts[c]++;
    return counts;
}, {});
let pairs: Record<string, number> = template
    .slice(0, -1)
    .map((_, i) => template.slice(i, i + 2).join(""))
    .reduce((pairs: Record<string, number>, p) => {
        if (!(p in pairs)) pairs[p] = 0;
        pairs[p]++;
        return pairs;
    }, {});

range(10).forEach(_ => {
    const newpairs = Object.entries(pairs).reduce((newpairs: Record<string, number>, [k, v]) => {
        const letter = rules[k];
        const x = k[0] + letter;
        const y = letter + k[1];

        if (!(letter in counts)) counts[letter] = 0;
        counts[letter] += v;

        if (!(x in newpairs)) newpairs[x] = 0;
        if (!(y in newpairs)) newpairs[y] = 0;
        newpairs[x] += v;
        newpairs[y] += v;

        return newpairs;
    }, {});
    pairs = newpairs;
});
let max = Object.keys(counts).reduce((max, c) => Math.max(max, counts[c]), 0);
let min = Object.keys(counts).reduce((min, c) => Math.min(min, counts[c]), max);
console.log("Part1:", max - min);

range(30).forEach(_ => {
    const newpairs = Object.entries(pairs).reduce((newpairs: Record<string, number>, [k, v]) => {
        const letter = rules[k];
        const x = k[0] + letter;
        const y = letter + k[1];

        if (!(letter in counts)) counts[letter] = 0;
        counts[letter] += v;

        if (!(x in newpairs)) newpairs[x] = 0;
        if (!(y in newpairs)) newpairs[y] = 0;
        newpairs[x] += v;
        newpairs[y] += v;

        return newpairs;
    }, {});
    pairs = newpairs;
});
max = Object.keys(counts).reduce((max, c) => Math.max(max, counts[c]), 0);
min = Object.keys(counts).reduce((min, c) => Math.min(min, counts[c]), max);
console.log("Part2:", max - min);
