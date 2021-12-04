import { readFileSync } from "fs";

import { range, sum } from "./utils";

type Row = (number | null)[];
type Board = Row[];

const parseInput = (): [number[], Board[]] => {
    const input = readFileSync("input/day04").toString().split("\n\n");
    const drawn = input[0].split(",").map(n => parseInt(n));
    const boards = input.slice(1).map(b =>
        b.split("\n").map(l =>
            l
                .split(" ")
                .map(n => parseInt(n))
                .filter(n => !isNaN(n))
        )
    );
    return [drawn, boards];
};

const boardComplete = (board: Board): boolean => {
    const complete = (xs: Row) => xs.every(x => x === null);
    const rowComplete = board.some(r => complete(r));
    const cols = range(0, board.length).map(c => board.map(l => l[c]));
    const colComplete = cols.some(c => complete(c));
    return rowComplete || colComplete;
};

const p1 = (drawn: number[], boards: Board[]): number | undefined => {
    for (const d of drawn) {
        for (let i = 0; i < boards.length; i++) {
            boards[i] = range(0, boards[i].length).map(j =>
                boards[i][j].map(n => (n !== d ? n : null))
            );

            if (boardComplete(boards[i])) {
                const sum = boards[i].reduce((sum, line) => {
                    const l = line.reduce((sum, x) => (sum || 0) + (x || 0));
                    return sum + (l || 0);
                }, 0);
                return d * sum;
            }
        }
    }
};

const p2 = (drawn: number[], boards: Board[]): number | undefined => {
    let result = 0;
    for (const d of drawn) {
        for (let i = 0; i < boards.length; i++) {
            boards[i] = range(0, boards[i].length).map(j =>
                boards[i][j].map(n => (n !== d ? n : null))
            );

            if (boardComplete(boards[i])) {
                result =
                    d *
                    boards[i].reduce((t, line) => {
                        return t + sum(line.filter(x => x !== null) as number[]);
                    }, 0);

                boards.splice(i, 1);
                i -= 1;
            }
        }
    }
    return result;
};

console.log("Part1:", p1(...parseInput()));
console.log("Part2:", p2(...parseInput()));
