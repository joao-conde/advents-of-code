import { range, readFileAsString, sum } from "./utils";

type Row = (number | null)[];
type Card = Row[];

const markCard = (card: Card, draw: number): Card =>
    card.map(row => row.map(n => (n !== draw ? n : null)));

const scoreCard = (card: Card, draw: number): number =>
    draw * sum(card.flat().filter(n => n !== null) as number[]);

const cardComplete = (card: Card): boolean => {
    const complete = (xs: Row) => xs.every(x => x === null);
    const rowComplete = card.some(r => complete(r));
    const colComplete = range(card.length)
        .map(c => card.map(r => r[c]))
        .some(c => complete(c));
    return rowComplete || colComplete;
};

const input = readFileAsString("input/day04").split("\n\n");
const drawn = input[0].split(",").map(n => parseInt(n));
const cards = input.slice(1).map(c =>
    c.split("\n").map(r =>
        r
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
