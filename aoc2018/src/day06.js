/* Link to problem: https://adventofcode.com/2018/day/6 */
const input = require('fs').readFileSync('../res/d06').toString().split('\n');

/* Regions count if above the threshold */
const SAFE_THRESHOLD = 10000;

/* Calculates Manhattan Distance */
const computeManhattanDistance = (x0, y0, x1, y1) => {
    return Math.abs(x0 - x1) + Math.abs(y0 - y1);
};

let safeSpots = 0; /* Spots above the threshold for P2 */
const infinite = new Set(); /* Set of points that extend to infinity */
const coords = input.map((el) => [parseInt(el.split(',')[0]) - 1, parseInt(el.split(',')[1]) - 1]);
const areas = Array(coords.length).fill(0); /* Count of cells for each point area */
const grid = Array(Math.max(...coords.map((el) => el[0]))+1).fill()
    .map(() => Array(Math.max(...coords.map((el) => el[1]))+1).fill('.'));

grid.forEach((row, r, grid) => {
    row.forEach((el, e) => {
        const distances = coords.map((coord) => {
            return computeManhattanDistance(r, e, coord[0], coord[1]);
        });

        if (distances.reduce((acc, cur) => acc + cur) < SAFE_THRESHOLD) safeSpots++;

        const minimalDistance = Math.min(...distances);
        const unique = distances.filter((dis) => dis == minimalDistance).length;
        grid[r][e] = (unique == 1 ? distances.indexOf(minimalDistance) : '.');

        /* If its an outside of the matrix that point extends to infinity */
        if (r == 0 || r == grid.length-1 || e == 0 || e == row.length-1) infinite.add(grid[r][e]);
    });
});

grid.forEach((row) => {
    row.forEach((el) => {
        if (el >= 0 && el < areas.length && !infinite.has(el)) areas[el]++;
    });
});

console.log('P1 - The size for dangerous coordinates of the largest finite area is ' + Math.max(...areas));
console.log('P2 - The size for safe coordinates of the largest finite area is ' + safeSpots);
