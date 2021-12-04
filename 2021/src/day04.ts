import { readFileSync } from "fs";

import { range, sum } from "./utils";

type Row = (number | null)[];
type Card = Row[];

const filterCard = (card: Card, draw: number): Card =>
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

const results = [];
for (const d of drawn) {
    for (let i = 0; i < cards.length; i++) {
        cards[i] = filterCard(cards[i], d);
        if (cardComplete(cards[i])) {
            results.push(scoreCard(cards[i], d));
            cards.splice(i, 1);
            i -= 1;
        }
    }
}

console.log("Part1:", results[0]);
console.log("Part2:", results[results.length - 1]);
