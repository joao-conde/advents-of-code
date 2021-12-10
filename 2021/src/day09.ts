import { readFileSync } from "fs";

import { mul, range, sum } from "./utils";

const hash = (i: number, j: number): string => [i, j].toString();

const isLowpoint = (heightmap: number[][], i: number, j: number): boolean => {
    const up = (heightmap[i - 1] || [])[j];
    const down = (heightmap[i + 1] || [])[j];
    const left = heightmap[i][j - 1];
    const right = heightmap[i][j + 1];
    return [up, down, left, right].filter(x => x !== undefined).every(x => x > heightmap[i][j]);
};

const lowpoints = (heightmap: number[][]): number[][] =>
    range(heightmap.length).reduce((points: number[][], i) => {
        const lowpoints = range(heightmap[i].length)
            .filter(j => isLowpoint(heightmap, i, j))
            .map(j => [i, j]);
        return [...points, ...lowpoints];
    }, []);

const fill = (
    heightmap: number[][],
    i: number,
    j: number,
    basins = new Set<string>()
): Set<string> => {
    if (basins.has(hash(i, j))) return basins;
    basins.add(hash(i, j));

    const lowpoint = heightmap[i][j];
    for (let c = j - 1; c >= 0; c--) {
        if (heightmap[i][c] <= lowpoint || heightmap[i][c] === 9) break;
        fill(heightmap, i, c, basins);
    }

    for (let c = j + 1; c < heightmap[0].length; c++) {
        if (heightmap[i][c] <= lowpoint || heightmap[i][c] === 9) break;
        fill(heightmap, i, c, basins);
    }

    for (let r = i - 1; r >= 0; r--) {
        if (heightmap[r][j] <= lowpoint || heightmap[r][j] === 9) break;
        fill(heightmap, r, j, basins);
    }

    for (let r = i + 1; r < heightmap.length; r++) {
        if (heightmap[r][j] <= lowpoint || heightmap[r][j] === 9) break;
        fill(heightmap, r, j, basins);
    }

    return basins;
};

const input = readFileSync("input/day09").toString().split("\n");
const heightmap = input.map(r => r.split("").map(h => parseInt(h)));

const points = lowpoints(heightmap);
const risk = sum(points.map(([i, j]) => heightmap[i][j] + 1));
console.log("Part1:", risk);

const basins = points.map(([i, j]) => fill(heightmap, i, j));
const sizes = basins.map(b => b.size).sort((a, b) => b - a);
console.log("Part2:", mul(sizes.slice(0, 3)));
