import { readFileSync } from "fs";

const basin = (
    heightmap: number[][],
    i: number,
    j: number,
    basinsxd123: Set<string>
): Set<string> => {
    if (basinsxd123.has([i, j].toString())) return basinsxd123;

    basinsxd123.add([i, j].toString());

    const lowpoint = heightmap[i][j];

    // left
    for (let c = j - 1; c >= 0; c--) {
        if (heightmap[i][c] < lowpoint || heightmap[i][c] === 9) break;

        basin(heightmap, i, c, basinsxd123);
        basinsxd123.add([i, c].toString());
    }

    // right
    for (let c = j + 1; c < heightmap[0].length; c++) {
        if (heightmap[i][c] < lowpoint || heightmap[i][c] === 9) break;
        basin(heightmap, i, c, basinsxd123);
        basinsxd123.add([i, c].toString());
    }

    // up
    for (let r = i - 1; r >= 0; r--) {
        if (heightmap[r][j] < lowpoint || heightmap[r][j] === 9) break;
        basin(heightmap, r, j, basinsxd123);
        basinsxd123.add([r, j].toString());
    }

    // down
    for (let r = i + 1; r < heightmap.length; r++) {
        if (heightmap[r][j] < lowpoint || heightmap[r][j] === 9) break;
        basin(heightmap, r, j, basinsxd123);
        basinsxd123.add([r, j].toString());
    }

    return basinsxd123;
};

const input = readFileSync("input/day09").toString().split("\n");
const heightmap = input.map(r => r.split("").map(h => parseInt(h)));

let risk = 0;
const lowpoints = [];
for (let i = 0; i < heightmap.length; i++) {
    for (let j = 0; j < heightmap[0].length; j++) {
        const prevLine = heightmap[i - 1] || [];
        const nextLine = heightmap[i + 1] || [];
        const low = [prevLine[j], nextLine[j], heightmap[i][j - 1], heightmap[i][j + 1]]
            .filter(x => x !== undefined)
            .every(x => x > heightmap[i][j]);

        if (low) risk += heightmap[i][j] + 1;
        if (low) lowpoints.push([i, j]);
    }
}

console.log(risk);

const basinSizes = lowpoints
    .map(([i, j]) => basin(heightmap, i, j, new Set()).size)
    .sort((a, b) => b - a);
console.log(basinSizes[0] * basinSizes[1] * basinSizes[2]);
