import { readFileSync } from "fs";

import { cartesian, sum } from "./utils";

const GRID_SIZE = 10;
const FLASH_THRESHOLD = 9;

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

const step = (octopuses: number[][]): [number, number[][]] => {
    // increment octopuses energy level
    octopuses = octopuses.map(r => r.map(o => o + 1));

    // increment adjacent octopuses energy
    // and count all flashers
    let flashes = 0;
    let flashers = getFlashers(octopuses);
    while (flashers.length > 0) {
        flashes += flashers.length;
        flash(octopuses, flashers);
        flashers = getFlashers(octopuses);
    }

    return [flashes, octopuses];
};

const input = readFileSync("input/day11").toString().split("\n");

const flashes = [];
let octopuses = input.map(r => r.split("").map(o => parseInt(o)));
while (true) {
    let newFlashes;
    [newFlashes, octopuses] = step(octopuses);
    flashes.push(newFlashes);
    if (newFlashes === 100) break;
}

console.log("Part1:", sum(flashes.slice(0, 100)));
console.log("Part2:", flashes.length);
