// import { car } from "./utils";
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

        if (plays1) {
            p1 = (p1 + move) % 10;
            score1 += p1 + 1;
        } else {
            p2 = (p2 + move) % 10;
            score2 += p2 + 1;
        }

        plays1 = !plays1;
        rolls += 3;
    }

    return {
        score1,
        score2,
        rolls
    };
};

type GameState = {
    score1: number;
    score2: number;
    p1: number;
    p2: number;
    plays1: boolean;
};

const play2 = (p1: number, p2: number): any => {

    const cache: Record<string, GameState> = {};

    let states: GameState[] = [
        {
            score1: 0,
            score2: 0,
            p1: p1,
            p2: p2,
            plays1: true
        }
    ];

    let p1wins = 0;
    let p2wins = 0;
    while (states.length > 0) {
        console.log(states.length)
        const nextStates: GameState[] = [];

        for (const state of states) {
            const { score1, score2, p1, p2, plays1 } = state;

            const k = JSON.stringify(state);
            if(k in cache) {
                const cached = cache[k]
                if (cached.score1 > cached.score2) p1wins++;
                else p2wins++;
                continue;
            }

            if (score1 >= 21 || score2 >= 21) {
                if (score1 > score2) p1wins++;
                else p2wins++;

                cache[k] = state

                continue;
            }

            const rolls = [1, 2, 3];
            for (const r1 of rolls) {
                for (const r2 of rolls) {
                    for (const r3 of rolls) {
                        const move = r1 + r2 + r3;
                        const nextp1 = plays1 ? (p1 + move) % 10 : p1;
                        const nextp2 = !plays1 ? (p2 + move) % 10 : p2;
                        const nextscore1 = plays1 ? score1 + 1 + nextp1 : score1;
                        const nextscore2 = !plays1 ? score2 + 1 + nextp2 : score2;
                        nextStates.push({
                            score1: nextscore1,
                            score2: nextscore2,
                            p1: nextp1,
                            p2: nextp2,
                            plays1: !plays1
                        });
                    }
                }
            }
        }
        states = nextStates;
    }

    return [p1wins, p2wins];
};

// const { score1, score2, rolls } = play(true, 4 - 1, 8 - 1, 0, 0, 1000); // -1 on coords
// console.log("Part1:", rolls * (score2 < score1 ? score2 : score1));

const x = play2(3, 7);
console.log(x);
console.log([444356092776315, 341960390180808]);
