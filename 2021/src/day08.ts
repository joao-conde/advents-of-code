import { readFileSync } from "fs";

const config = (patterns: string[][]): string[][] => {
    const one = patterns.find(p => p.length === 2)!;
    const four = patterns.find(p => p.length === 4)!;
    const seven = patterns.find(p => p.length === 3)!;
    const eight = patterns.find(p => p.length === 7)!;

    // find out 9
    const length6 = patterns.filter(p => p.length === 6);
    const missing = length6.flatMap(p => eight.filter(pi => !p.includes(pi)));
    const side4 = missing.find(m => !four.includes(m))!;
    const nine = eight.filter(i => i !== side4);

    // find 6
    const side2 = missing.find(m => one.includes(m));
    const six = eight.filter(i => i !== side2);

    // find 0
    const side3 = missing.find(m => m !== side4 && m !== side2);
    const zero = eight.filter(i => i !== side3);

    // find 5
    const five = eight.filter(i => i !== side2 && i !== side4);

    const side5 = one.find(i => six.includes(i));
    const side1 = four.find(i => i !== side2 && i !== side3 && i !== side5);

    const three = eight.filter(i => i !== side1 && i !== side4);
    const two = three.concat([side4]).filter(i => i !== side5);

    return [zero, one, two, three, four, five, six, seven, eight, nine];
};

const input = readFileSync("input/day08").toString().split("\n");

const p1 = input.reduce((unique, l) => {
    const [_patterns, output] = l.split("|");
    const lengths = output
        .trim()
        .split(" ")
        .map(o => o.length);
    return unique + lengths.filter(l => [2, 3, 4, 7].includes(l)).length;
}, 0);
console.log("Part1:", p1);

const p2 = input.reduce((sum, l) => {
    const [patterns, output] = l.split("|");

    const conf = config(
        patterns
            .trim()
            .split(" ")
            .map(p => p.split(""))
    );

    const code = output
        .trim()
        .split(" ")
        .reduce((code, o) => {
            const chars = o.split("");
            const i = conf.findIndex(k =>
                k.every(x => chars.includes(x) && chars.every(x => k.includes(x)))
            );
            return `${code}${i}`;
        }, "");

    return sum + parseInt(code);
}, 0);
console.log("Part2:", p2);
