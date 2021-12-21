// import { readFileAsString } from "./utils";
// const input = readFileAsString("input/day21");

const play = (
    plays1: boolean,
    p1: number,
    p2: number,
    score1: number,
    score2: number,
    max: number
): { score1: number; score2: number; rolls: number } => {
    let rolls = 0;
    while (score1 < max && score2 < max) {
        const move = (rolls % 100) + ((rolls + 1) % 100) + ((rolls + 2) % 100) + 3;

        ({ score1, score2, p1, p2 } = playIter(plays1, p1, p2, score1, score2, move));

        plays1 = !plays1;
        rolls += 3;
    }

    return {
        score1,
        score2,
        rolls
    };
};

const playIter = (
    plays1: boolean,
    p1: number,
    p2: number,
    score1: number,
    score2: number,
    move: number
): { score1: number; score2: number; p1: number; p2: number } => {
    if (plays1) {
        p1 = (p1 + move) % 10;
        score1 += p1 + 1;
    } else {
        p2 = (p2 + move) % 10;
        score2 += p2 + 1;
    }

    return {
        score1,
        score2,
        p1,
        p2
    };
};

const cache: Record<string, number> = {};

const play2 = (
    plays1: number,
    p1: number,
    p2: number,
    score1: number,
    score2: number,
    max: number,
    move: number
): number => {
    const k = `${p1}${p2}${score1}${score2}${move}`;

    if (k in cache) return cache[k];

    if (score1 >= max || score2 >= max) {
        cache[k] = score1 > score2 ? 1 : 0;
        return cache[k];
    }

    if (plays1 <= 2) {
        p1 = (p1 + move) % 10;
    } else if (plays1 > 2 && plays1 <= 5) {
        p2 = (p2 + move) % 10;
    }

    if (plays1 === 2) {
        score1 += p1 + 1;
    } else if (plays1 === 5) {
        score2 += p2 + 1;
    }

    const y =
        play2((plays1 + 1) % 6, p1, p2, score1, score2, 21, 1) +
        play2((plays1 + 1) % 6, p1, p2, score1, score2, 21, 2) +
        play2((plays1 + 1) % 6, p1, p2, score1, score2, 21, 3);

    cache[k] = y;

    return y;
};

const { score1, score2, rolls } = play(true, 3, 7, 0, 0, 1000); // -1 on coords
console.log("Part1:", rolls * (score2 < score1 ? score2 : score1));

console.log(
    play2(0, 3, 7, 0, 0, 21, 1) + play2(0, 3, 7, 0, 0, 21, 2) + play2(0, 3, 7, 0, 0, 21, 3)
);
console.log(444356092776315, "expected");
