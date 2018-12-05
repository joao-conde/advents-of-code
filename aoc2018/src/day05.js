// Link to problem: https://adventofcode.com/2018/day/5
const input = require('fs').readFileSync('../res/d05').toString();

const unitsReact = (unit1, unit2) => {
	return (unit1.toUpperCase() == unit2.toUpperCase() && unit1 != unit2);
};

const deleteUnit = (polymer, unitIdx) => {
	return polymer.slice(0, unitIdx) + polymer.slice(unitIdx + 1, polymer.length);
};

const reactPolymer = (polymer) => {
	let i = 0;
	while (i < polymer.length - 1) {
		if (unitsReact(polymer[i], polymer[i+1])) {
			polymer = deleteUnit(deleteUnit(polymer, i+1), i);
			i = (i >= 2 ? i-2 : 0);
		} else i++;
	}
	return polymer;
};

console.log('P1 - Units remaining after polymer reactions: ' + reactPolymer(input).length);
