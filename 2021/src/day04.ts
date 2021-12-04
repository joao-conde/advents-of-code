import { readFileSync } from "fs";

import { range, sum } from "./utils";

type Row = (number | null)[];
type Card = Row[];

const markCard = (card: Card, draw: number): Card =>
    card.map(row => row.map(n => (n !== draw ? n : null)));

const cardComplete = (card: Card): boolean => {
    const complete = (xs: Row) => xs.every(x => x === null);
    const rowComplete = card.some(r => complete(r));
    const colComplete = range(0, card.length)
        .map(c => card.map(l => l[c]))
        .some(c => complete(c));
    return rowComplete || colComplete;
};

const scoreCard = (card: Card, draw: number): number =>
    draw * sum(card.flat().filter(n => n !== null) as number[]);

const input = readFileSync("input/day04").toString().split("\n\n");
const drawn = input[0].split(",").map(n => parseInt(n));
const cards: Card[] = input.slice(1).map(b =>
    b.split("\n").map(l =>
        l
            .split(" ")
            .map(n => parseInt(n))
            .filter(n => !isNaN(n))
    )
);

const scores = drawn.reduce(
    (acc: { cards: Card[]; scores: number[] }, d: number) => {
        const cards = acc.cards.map(card => markCard(card, d));
        const scores = cards.filter(card => cardComplete(card)).map(card => scoreCard(card, d));
        return {
            cards: cards.filter(card => !cardComplete(card)),
            scores: acc.scores.concat(scores)
        };
    },
    { cards: cards, scores: [] }
).scores;
console.log("Part1:", scores[0]);
console.log("Part2:", scores[scores.length - 1]);
