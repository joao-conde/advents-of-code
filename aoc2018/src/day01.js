//Link to problem: https://adventofcode.com/2018/day/1
const fs = require('fs');
const input = fs.readFileSync('../res/d01').toString().split('\n');

solveP1 = () => {
	console.log('Resulting frequency is ' + input.map(Number).reduce((acc, currVal) => acc + currVal));
};

solveP2 = () => {
	const previousFreqVals = new Set();

	let frequency = 0, i = 0;
	while(!previousFreqVals.has(frequency)){
		previousFreqVals.add(frequency);
		
		frequency += parseInt(input[i]);

		if(i + 1 == input.length)
			i = 0;
		else
			i++;
	}

	console.log('First repeated frequency is ' + frequency);
};

solveP1();
solveP2();