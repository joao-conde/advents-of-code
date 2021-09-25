const input = require("fs").readFileSync("input/day11").toString();

const puzzleInput = parseInt(input);

const computePowerLvl = (x, y, serial) => {
    const rackID = x + 10;
    const powerLvl = (rackID * y + serial) * rackID;
    return ((powerLvl % 1000) - (powerLvl % 100)) / 100 - 5;
};

// tests
// console.log(computePowerLvl(3, 5, 8));
// console.log(computePowerLvl(122, 79, 57));
// console.log(computePowerLvl(217, 196, 39));
// console.log(computePowerLvl(101, 153, 71));
