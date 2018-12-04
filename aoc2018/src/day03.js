// Link to problem: https://adventofcode.com/2018/day/3
const input = require('fs').readFileSync('../res/d03').toString().split('\n');

const fillFabric = (fabric, [x, y], [width, height]) => {
	for (let i = x; i < x + width; i++) {
		for (let j = y; j < y + height; j++) {
			const cellID = i + '-' + j;
			if (cellID in fabric) {
				fabric[cellID]++;
			} else {
				fabric[cellID] = 1;
			}
		}
	}
};

const fabric = {};
let conflicts = 0;
input.forEach((claim) => {
	const claimCoords = claim.split(' ')[2].split(',').map((x) => parseInt(x));
	const claimDimensions = claim.split(' ')[3].split('x').map((x) => parseInt(x));
	fillFabric(fabric, claimCoords, claimDimensions);
});
for (const cell in fabric) if (fabric[cell] >= 2) conflicts++;
console.log('P1: ' + conflicts + ' conflicting fabric inches');
