// Link to problem: https://adventofcode.com/2018/day/6
const input = require('fs').readFileSync('../res/d06').toString().split('\n');

const coords = input.map((el, i) => [String.fromCharCode('A'.charCodeAt(0) + i), parseInt(el.split(',')[0]), parseInt(el.split(',')[1])]);

const largerX = Math.max(...coords.map(el => el[0]));
const largerY = Math.max(...coords.map(el => el[1]));

//check & add the 'borders'
places = new Set();

const infinite = coords.forEach(el => {isFunc(el) ? places.add(el) : null} );

console.log(infinite);


//top line
//bottom line
//left col
//right col


// console.log(input);