/* Link to problem: https://adventofcode.com/2018/day/3 */
const input = require('fs').readFileSync('../res/d03').toString().split('\n');

/* Adds claim ID's to each fabric cell */
const updateFabric = (fabric, id, [x, y], [width, height]) => {
    for (let i = x; i < x + width; i++) {
        for (let j = y; j < y + height; j++) {
            const cellID = i + '-' + j;
            if (cellID in fabric) {
                fabric[cellID].push(id);
            } else {
                fabric[cellID] = [id];
            }
        }
    }
};

const fabric = {}, ids = [];
input.forEach((claim) => {
    ids.push(claim.split(' ')[0]);
    updateFabric(fabric, claim.split(' ')[0],
        claim.split(' ')[2].split(',').map((x) => parseInt(x)),
        claim.split(' ')[3].split('x').map((x) => parseInt(x)));
});

/*
 * Calculates conflicts or reused area from fabric and removes
 * areas with multiple claims, leaving the only that does not
 */
let conflicts = 0;
for (const cell in fabric) {
    if (fabric[cell].length >= 2) {
        conflicts++;
        fabric[cell].forEach((id) => {
            const idx = ids.indexOf(id); if (idx != -1) ids.splice(idx, 1);
        });
    }
}

console.log('P1 - ' + conflicts + ' conflicting fabric inches');
console.log('P2 - ID of non-overlapping claim is ' + ids);
