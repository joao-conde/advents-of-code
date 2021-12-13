import { range, readFileAsString, Set } from "./utils";

const input = readFileAsString("input/day13").split("\n\n");
const folds: [string, number][] = input[1]
    .split("\n")
    .map(l => l.split(" ")[2].split("="))
    .map(([axis, val]) => [axis, parseInt(val)]);
const dots = input[0]
    .split("\n")
    .map(l => l.split(",").map(x => parseInt(x)))
    .reduce((dots, point) => dots.add(point as [number, number]), new Set<[number, number]>());

const fold = (dots: Set<[number, number]>, [axis, value]: [string, number]) => {
    dots.values()
        .filter(([x, y]) => axis === "y" && y >= value)
        .forEach(([x, y]) => dots.delete([x, y]).add([x, 2 * value - y]));

    dots.values()
        .filter(([x, y]) => axis === "x" && x >= value)
        .forEach(([x, y]) => dots.delete([x, y]).add([2 * value - x, y]));
};

fold(dots, folds[0]);
console.log("Part1:", dots.size());

console.log("Part2:")
folds.slice(1).forEach(([axis, value]) => fold(dots, [axis, value]));
range(6).map(x => console.log(range(40).map(y => dots.has([y, x]) ? "#" : " ").join("")))
