import { readFileSync } from "fs";

import { range, scan } from "./utils";

const generateCoords = (x1: number, y1: number, x2: number, y2: number): number[][] => {
    const incx = x1 < x2 ? 1 : x1 > x2 ? -1 : 0;
    const incy = y1 < y2 ? 1 : y1 > y2 ? -1 : 0;
    const size = Math.max(Math.abs(x1 - x2), Math.abs(y1 - y2));
    const rn = range(0, size);
    const coords = scan(rn, [x1, y1], ([x, y]) => [x + incx, y + incy]);
    return coords;
};

const overlaps = (lines: number[][][], diagonals: boolean): number => {
    const overlaps = new Map<string, number>();
    lines.forEach(l => {
        if (!diagonals && l[0][0] !== l[1][0] && l[0][1] !== l[1][1]) return;
        const coords = generateCoords(l[0][0], l[0][1], l[1][0], l[1][1]);
        coords
            .map(p => p.toString())
            .forEach(p => {
                overlaps.set(p, (overlaps.get(p) || 0) + 1);
            });
    });
    return Array.from(overlaps.values()).filter(x => x >= 2).length;
};

const input = readFileSync("input/day05").toString().split("\n");
const lines = input.map(l => l.split("->").map(p => p.split(",").map(x => parseInt(x))));
console.log("Part1:", overlaps(lines, false));
console.log("Part2:", overlaps(lines, true));
