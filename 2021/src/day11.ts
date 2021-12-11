import { readFileSync } from "fs";

import { cartesian } from "./utils";

const GRID_SIZE = 10;

const step = (octopuses: number[][]): [number, number[][]] => {
    octopuses = octopuses.map(r => r.map(o => o + 1));

    let flashes = 0;

    let flashers = octopuses
        .map((r, i) => r.map((o, j) => [i, j, o]))
        .flat()
        .filter(([_i, _j, o]) => o > 9);

    while (flashers.length > 0) {
        flashes += flashers.length;

        flashers.forEach(([i, j, o]) => {
            const cols = [-1, 0, 1].map(x => x + j).filter(c => c >= 0 && c < GRID_SIZE);
            const rows = [-1, 0, 1].map(x => x + i).filter(r => r >= 0 && r < GRID_SIZE);

            const coords = [
                ...new Set(cartesian(rows, cols).filter(([r, c]) => r !== i || c !== j))
            ];

            coords.forEach(([r, c]) => {
                if (octopuses[r][c] !== 0) {
                    octopuses[r][c]++;
                }
            });
        });

        flashers.forEach(([i, j]) => {
            octopuses[i][j] = 0;
        });

        flashers = octopuses
            .map((r, i) => r.map((o, j) => [i, j, o]))
            .flat()
            .filter(([_i, _j, o]) => o > 9);
    }

    return [flashes, octopuses];
};

const input = readFileSync("input/day11").toString().split("\n");
let octopuses = input.map(r => r.split("").map(o => parseInt(o)));

let flashes = 0;
for (let i = 0; i < 100; i++) {
    let newFlashes;
    [newFlashes, octopuses] = step(octopuses);
    flashes += newFlashes;
}
console.log("Part1:", flashes);

octopuses = input.map(r => r.split("").map(o => parseInt(o)));
flashes = 0;
let i = 0;
while (flashes !== 100) {
    [flashes, octopuses] = step(octopuses);
    i++;
}
console.log("Part2:", i);
