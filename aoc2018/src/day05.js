/* Link to problem: https://adventofcode.com/2018/day/5 */
const input = require('fs').readFileSync('../res/d05').toString();

/* Tests if 2 units react i.e. they are the same type but different polarity */
const unitsReact = (unit1, unit2) => {
    return (unit1.toUpperCase() == unit2.toUpperCase() && unit1 != unit2);
};

/* Deletes an unit from the polymer */
const deleteUnit = (polymer, unitIdx) => {
    return polymer.slice(0, unitIdx) + polymer.slice(unitIdx + 1, polymer.length);
};

/*
 * Causes chain reactions across the polymer, reducing it, i.e.
 * it removes pairs of same letters with difference casing.
 */
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

/*
 * Remove pairs of same letters with difference casing after removing all
 * instances of specific letter. Identify which combination generates the
 * smallest length
 */
const polymerLengths = [...new Set(input)].map((unique) => {
    return reactPolymer(input.split('').filter((letter) => {
        return (letter.toUpperCase() != unique.toUpperCase());
    }).join('')).length;
});

console.log('P1 - Units remaining after polymer reactions: ' + reactPolymer(input).length);
console.log('P2 - Shortest polymer length: ' + Math.min(...polymerLengths));
