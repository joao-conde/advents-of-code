const input = require("fs").readFileSync("input/day03").toString().split("\n");

const updateFabric = (fabric, id, [x, y], [width, height]) => {
    for (let i = x; i < x + width; i += 1) {
        for (let j = y; j < y + height; j += 1) {
            const cellID = `${i}-${j}`;
            if (cellID in fabric) {
                fabric[cellID].push(id);
            } else {
                fabric[cellID] = [id];
            }
        }
    }
};

const fabric = {};
const ids = [];
input.forEach(claim => {
    ids.push(claim.split(" ")[0]);
    updateFabric(
        fabric,
        claim.split(" ")[0],
        claim
            .split(" ")[2]
            .split(",")
            .map(x => parseInt(x)),
        claim
            .split(" ")[3]
            .split("x")
            .map(x => parseInt(x))
    );
});

let conflicts = 0;
for (const cell in fabric) {
    if (fabric[cell].length >= 2) {
        conflicts += 1;
        fabric[cell].forEach(id => {
            const idx = ids.indexOf(id);
            if (idx !== -1) ids.splice(idx, 1);
        });
    }
}

console.log(`P1 - ${conflicts} conflicting fabric inches`);
console.log(`P2 - ID of non-overlapping claim is ${ids}`);
