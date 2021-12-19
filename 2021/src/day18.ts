import { readFileAsString, windows } from "./utils";

type SFNumber = [number, number][];

const build = (n: string): SFNumber => {
    const l: SFNumber = [];
    let lvl = 0;
    for (let i = 0; i < n.length; i++) {
        if (n[i] === "[") lvl += 1;
        else if (n[i] === "]") lvl -= 1;
        else if (n[i] === ",") continue;
        else {
            const num = isNaN(parseInt(n[i] + n[i + 1]))
                ? parseInt(n[i])
                : parseInt(n[i] + n[i + 1]);
            l.push([num, lvl]);
        }
    }
    return l;
};

const add = (n1: SFNumber, n2: SFNumber): SFNumber => [
    ...(n1.map(([v1, lvl1]) => [v1, lvl1 + 1]) as SFNumber),
    ...(n2.map(([v2, lvl2]) => [v2, lvl2 + 1]) as SFNumber)
];

const explode = (n: SFNumber): boolean => {
    for (const [i, [[v1, lvl1], [v2, lvl2]]] of windows(n, 2).entries()) {
        if (lvl1 === lvl2 && lvl1 > 4) {
            if (n[i - 1]) n[i - 1][0] += v1;
            if (n[i + 2]) n[i + 2][0] += v2;
            n.splice(i, 2, [0, 4]);
            // n.splice(i, 0, [0, 4]);
            return true;
        }
    }
    return false;
};

const split = (n: SFNumber): boolean => {
    for (const [i, [value, level]] of n.entries()) {
        if (value >= 10) {
            n.splice(i, 1, [Math.floor(value / 2), level + 1], [Math.ceil(value / 2), level + 1]);
            return true;
        }
    }
    return false;
};

const reduce = (n: SFNumber): SFNumber => {
    while (true) {
        if (explode(n)) continue;
        if (split(n)) continue;
        return n;
    }
};

const magnitude = (n: SFNumber) => {
    while (n.length > 1) {
        const i = windows(n, 2).findIndex(([[_v1, lvl1], [_v2, lvl2]]) => lvl1 === lvl2);
        const v = 3 * n[i][0] + 2 * n[i + 1][0];
        n.splice(i, 2, [v, n[i][1] - 1]);
    }
    return n[0][0];
};

const input = readFileAsString("input/day18");
const builds = input.split("\n").map(line => reduce(build(line)));
const begin = reduce(add(reduce(builds[0]), reduce(builds[1])));
const num = builds.slice(2).reduce((sum: SFNumber, n) => {
    return reduce(add(sum, n));
}, begin);

console.log(magnitude(num));
