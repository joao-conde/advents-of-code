import { mul, range, readFileAsString, sum, JCSet as Set } from "./utils";

const lowpoints = (heightmap: number[][]): number[][] =>
    range(heightmap.length).reduce((points: number[][], i) => {
        const lowpoints = range(heightmap[i].length)
            .filter(j => isLowpoint(heightmap, i, j))
            .map(j => [i, j]);
        return [...points, ...lowpoints];
    }, []);

const isLowpoint = (heightmap: number[][], i: number, j: number): boolean =>
    [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0]
    ]
        .map(([r, c]) => [r + i, c + j])
        .filter(([r, c]) => r >= 0 && r < heightmap.length && c >= 0 && c < heightmap[0].length)
        .every(([r, c]) => heightmap[r][c] > heightmap[i][j]);

const fill = (
    heightmap: number[][],
    i: number,
    j: number,
    basins = new Set<[number, number]>()
): Set<[number, number]> => {
    if (basins.has([i, j])) return basins;
    basins.add([i, j]);

    const lowpoint = heightmap[i][j];
    const stop = (r: number, c: number): boolean =>
        heightmap[r][c] <= lowpoint || heightmap[r][c] === 9;

    for (let c = j - 1; c >= 0; c--) {
        if (stop(i, c)) break;
        fill(heightmap, i, c, basins);
    }

    for (let c = j + 1; c < heightmap[0].length; c++) {
        if (stop(i, c)) break;
        fill(heightmap, i, c, basins);
    }

    for (let r = i - 1; r >= 0; r--) {
        if (stop(r, j)) break;
        fill(heightmap, r, j, basins);
    }

    for (let r = i + 1; r < heightmap.length; r++) {
        if (stop(r, j)) break;
        fill(heightmap, r, j, basins);
    }

    return basins;
};

const input = readFileAsString("input/day09").split("\n");
const heightmap = input.map(r => r.split("").map(h => parseInt(h)));

const points = lowpoints(heightmap);
const risk = sum(points.map(([i, j]) => heightmap[i][j] + 1));
console.log("Part1:", risk);

const basins = points.map(([i, j]) => fill(heightmap, i, j));
const sizes = basins.map(b => b.size).sort((a, b) => b - a);
console.log("Part2:", mul(sizes.slice(0, 3)));
