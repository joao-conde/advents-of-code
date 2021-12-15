import { copy, range, readFileAsString } from "./utils";

const shortestPathLen = (risks: number[][]): number => {
    const height = risks.length;
    const width = risks[0].length;

    const distances: number[][] = copy(risks).map(r => r.map(_ => Infinity));
    distances[0][0] = 0;

    const queue: [[number, number], number][] = [[[0, 0], 0]];
    while (queue.length > 0) {
        const [[x1, y1], distance] = queue.pop()!;

        const neighbors = [
            [x1 + 1, y1],
            [x1 - 1, y1],
            [x1, y1 + 1],
            [x1, y1 - 1]
        ].filter(([x1, y1]) => x1 >= 0 && x1 < height && y1 >= 0 && y1 < width);

        neighbors.forEach(([x2, y2]) => {
            const cost = distance + risks[x2][y2];
            if (cost < distances[x2][y2]) {
                distances[x2][y2] = cost;
                queue.push([[x2, y2], cost]);
                queue.sort((a, b) => b[1] - a[1]);
            }
        });
    }
    return distances[height - 1][width - 1];
};

const expand = (grid: number[][]) => {
    const width = grid[0].length;
    const height = grid.length;

    const expandRow = (row: number[]) =>
        range(4).reduce((row, _) => {
            const next = row.slice(-width).map(x => (x + 1 > 9 ? 1 : x + 1));
            return row.concat(next);
        }, row);

    const firstBlock = range(height).map(i => expandRow(grid[i]));
    const blocks = range(4).reduce((block, _) => {
        const next = block.slice(-height).map(r => r.map(x => (x + 1 > 9 ? 1 : x + 1)));
        return block.concat(next);
    }, firstBlock);
    return blocks;
};

const input = readFileAsString("input/day15").split("\n");
const risks = input.map(r => r.split("").map(x => parseInt(x)));
console.log("Part1:", shortestPathLen(risks));
console.log("Part2:", shortestPathLen(expand(risks)));
