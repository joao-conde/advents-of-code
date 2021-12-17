import { cartesian, range, readFileAsString } from "./utils";

const heights = (vx: number, vy: number): number[] => {
    const xs = [0];
    const ys = [0];
    while (true) {
        const x = xs[xs.length - 1];
        const y = ys[ys.length - 1];

        if (x >= x1 && x <= x2 && y >= y1 && y <= y2) return ys;
        if (x > x2 || y < y1) return [];

        xs.push(x + vx);
        ys.push(y + vy);
        vx = vx > 0 ? vx - 1 : vx < 0 ? vx + 1 : vx;
        vy = vy - 1;
    }
};

const REGEX = /target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)/g;
const SEARCH_RANGE = range(-300, 0).concat(range(300));

const input = readFileAsString("input/day17");
const [x1, x2, y1, y2] = REGEX.exec(input)!
    .slice(1, 5)
    .map(x => parseInt(x));

const ys = cartesian(SEARCH_RANGE, SEARCH_RANGE)
    .map(([vx, vy]) => heights(vx, vy))
    .filter(p => p.length !== 0);
console.log("Part1:", Math.max(...ys.flat()));
console.log("Part2:", ys.length);
