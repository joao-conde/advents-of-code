import { cartesian, range, readFileAsString } from "./utils";

type GameSummary = {
    s1: number;
    s2: number;
    rolls?: number;
};

type Cache = Record<string, GameSummary>;

type Die = (
    p1: number,
    p2: number,
    s1: number,
    s2: number,
    options: { rolls: number; cache: Cache }
) => GameSummary;

const play = (die: Die, p1: number, p2: number): GameSummary => {
    const cache: Record<string, GameSummary> = {};
    return die(p1 - 1, p2 - 1, 0, 0, { rolls: 0, cache: cache });
};

const deterministic = (
    p1: number,
    p2: number,
    s1: number,
    s2: number,
    { rolls }: { rolls: number }
): GameSummary => {
    if (s1 >= 1000 || s2 >= 1000) return { s1: s1, s2: s2, rolls: rolls };
    const move = (rolls % 100) + ((rolls + 1) % 100) + ((rolls + 2) % 100) + 3;
    const nextp1 = (p1 + move) % 10;
    const nextscore1 = s1 + 1 + nextp1;
    return deterministic(p2, nextp1, s2, nextscore1, { rolls: rolls + 3 });
};

const quantum = (
    p1: number,
    p2: number,
    s1: number,
    s2: number,
    { cache }: { cache: Cache }
): GameSummary => {
    if (s1 >= 21) return { s1: 1, s2: 0 };
    if (s2 >= 21) return { s1: 0, s2: 1 };

    const k = `${p1};${p2};${s1};${s2}`;
    if (k in cache) return cache[k];

    const counts = cartesian(range(1, 4), range(1, 4), range(1, 4)).reduce(
        (acc, [r1, r2, r3]) => {
            const nextp1 = (p1 + r1 + r2 + r3) % 10;
            const nextscore1 = s1 + 1 + nextp1;
            const summary = quantum(p2, nextp1, s2, nextscore1, { cache: cache });
            return { s1: acc.s1 + summary.s2, s2: acc.s2 + summary.s1 };
        },
        { s1: 0, s2: 0 }
    ) as GameSummary;

    cache[k] = counts;
    return counts;
};

const REGEX = /Player \d starting position: (?<pos>\d+)/;

const input = readFileAsString("input/day21");
const [p1, p2] = input
    .split("\n")
    .map(line => new RegExp(REGEX).exec(line)!.groups!.pos)
    .map(pos => parseInt(pos)) as [number, number];

let { s1, s2, rolls } = play(deterministic, p1, p2);
console.log("Part1:", rolls! * (s2 < s1 ? s2 : s1));

({ s1, s2, rolls } = play(quantum, p1, p2));
console.log("Part2:", s2 > s1 ? s2 : s1);
