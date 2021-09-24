const input = require('fs').readFileSync('../res/d01').toString().split('\n');

const resultingFreq = input.map(Number).reduce((acc, currVal) => acc + currVal);
console.log(`P1 - Resulting frequency is ${resultingFreq}`);

const previousFreqVals = new Set();
let frequency = 0;
let i = 0;
while (!previousFreqVals.has(frequency)) {
  previousFreqVals.add(frequency);
  frequency += parseInt(input[i]);
  if (i + 1 == input.length) i = 0;
  else i++;
}
console.log(`P2 - First repeated frequency is ${frequency}`);
