import { readFileSync } from "fs";

import { cartesian, sum } from "./utils";

const GRID_SIZE = 10;

const flashersfn = (octopuses: number[][]): number[][] =>
    octopuses
        .map((r, i) => r.map((o, j) => [i, j, o]))
        .flat()
        .filter(([_i, _j, o]) => o > 9);

const flash = (octopuses: number[][], flashers: number[][]) => {
    flashers
        .flatMap(([i, j, o]) => {
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
    octopuses = octopuses.map(r => r.map(o => o + 1));

    let flashes = 0;
    let flashers = flashersfn(octopuses);
    while (flashers.length > 0) {
        flashes += flashers.length;
        flash(octopuses, flashers);
        flashers = flashersfn(octopuses);
    }

    return [flashes, octopuses];
};

const input = readFileSync("input/day11").toString().split("\n");
let octopuses = input.map(r => r.split("").map(o => parseInt(o)));

const flashes = [];
while (true) {
    let newFlashes;
    [newFlashes, octopuses] = step(octopuses);
    flashes.push(newFlashes);
    if (newFlashes === 100) break;
}

console.log(sum(flashes.slice(0, 100)), flashes.length);
