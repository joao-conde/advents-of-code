import { readFileSync } from "fs";

import { range, scan } from "./utils";

const linePoints = ([[x1, y1], [x2, y2]]: number[][]): number[][] => {
    const incx = x1 < x2 ? 1 : x1 > x2 ? -1 : 0;
    const incy = y1 < y2 ? 1 : y1 > y2 ? -1 : 0;
    const size = Math.max(Math.abs(x1 - x2), Math.abs(y1 - y2));
    return scan(range(0, size), [x1, y1], ([x, y]) => [x + incx, y + incy]);
};

const overlaps = (lines: number[][][]): number => {
    const overlaps = new Map<string, number>();
    const points = lines.flatMap(l => linePoints(l));
    points.map(p => p.toString()).forEach(p => overlaps.set(p, (overlaps.get(p) || 0) + 1));
    return Array.from(overlaps.values()).filter(x => x >= 2).length;
};

const input = readFileSync("input/day05").toString().split("\n");
const lines = input.map(l => l.split("->").map(p => p.split(",").map(x => parseInt(x))));
const straight = lines.filter(([[x1, y1], [x2, y2]]) => x1 === x2 || y1 === y2);
console.log("Part1:", overlaps(straight));
console.log("Part2:", overlaps(lines));