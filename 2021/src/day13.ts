import { range, readFileAsString, JCSet as Set } from "./utils";

const fold = (dots: Set<[number, number]>, [axis, value]: [string, number]) => {
    const values = dots.array();
    values
        .filter(([_, y]) => axis === "y" && y >= value)
        .forEach(([x, y]) => dots.add([x, 2 * value - y]).delete([x, y]));

    values
        .filter(([x, _]) => axis === "x" && x >= value)
        .forEach(([x, y]) => dots.add([2 * value - x, y]).delete([x, y]));
};

const input = readFileAsString("input/day13").split("\n\n");
const folds: [string, number][] = input[1]
    .split("\n")
    .map(l => l.split(" ")[2].split("="))
    .map(([axis, val]) => [axis, parseInt(val)]);
const dots = input[0]
    .split("\n")
    .map(l => l.split(",").map(x => parseInt(x)))
    .reduce((dots, point) => dots.add(point as [number, number]), new Set<[number, number]>());

fold(dots, folds[0]);
console.log("Part1:", dots.size);

console.log("Part2:");
folds.slice(1).forEach(f => fold(dots, f));
range(6).forEach(x => {
    const line = range(40)
        .map(y => (dots.has([y, x]) ? "#" : " "))
        .join("");
    console.log(line);
});
