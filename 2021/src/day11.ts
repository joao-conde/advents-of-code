import { readFileSync } from "fs";

import { cartesian, copy, range } from "./utils";

const GRID_SIZE = 10;
const FLASH_THRESHOLD = 9;

const step = (octopuses: number[][]): number => {
    // increment octopuses energy level
    cartesian(range(GRID_SIZE), range(GRID_SIZE)).forEach(([r, c]) => octopuses[r][c]++);

    // increment adjacent octopuses energy and count flashers
    let flashes = 0;
    let flashers = getFlashers(octopuses);
    while (flashers.length > 0) {
        flashes += flashers.length;
        flash(octopuses, flashers);
        flashers = getFlashers(octopuses);
    }
    return flashes;
};

const getFlashers = (octopuses: number[][]): number[][] =>
    octopuses
        .map((r, i) => r.map((o, j) => [i, j, o]))
        .flat()
        .filter(([i, j, o]) => o > FLASH_THRESHOLD);

const flash = (octopuses: number[][], flashers: number[][]) => {
    flashers
        .flatMap(([i, j]) => {
            const cols = [-1, 0, 1].map(x => x + j).filter(c => c >= 0 && c < GRID_SIZE);
            const rows = [-1, 0, 1].map(x => x + i).filter(r => r >= 0 && r < GRID_SIZE);
            const coords = cartesian(rows, cols).filter(([r, c]) => r !== i || c !== j);
            return coords;
        })
        .filter(([r, c]) => octopuses[r][c] !== 0)
        .forEach(([r, c]) => octopuses[r][c]++);

    flashers.forEach(([i, j]) => (octopuses[i][j] = 0));
};

const p1 = (octopuses: number[][]): number =>
    range(100).reduce(flashes => flashes + step(octopuses), 0);

const p2 = (octopuses: number[][]): number => {
    let i = 1;
    while (step(octopuses) !== 100) i++;
    return i;
};

const input = readFileSync("input/day11").toString().split("\n");
const octopuses = input.map(r => r.split("").map(o => parseInt(o)));

console.log("Part1:", p1(copy(octopuses)));
console.log("Part2:", p2(octopuses));
