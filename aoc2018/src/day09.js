/* Link to problem: https://adventofcode.com/2018/day/9 */
const input = require('fs').readFileSync('../res/d09').toString();

const MULTIPLE = 23;

const numMarbles = parseInt(input.match('[0-9]*(?= points)')[0]); //not counting 0
const numPlayers = parseInt(input.match('[0-9]*(?= players)')[0]); 

const scores = new Array(numPlayers).fill(0);
let marbles = [0], curMarble = 0, curPlayer = 0;
for(let marbleToAdd = 1; marbleToAdd <= numMarbles; marbleToAdd++){
	if(marbleToAdd % MULTIPLE != 0){
		const left = (curMarble + 1) % marbles.length;
		marbles.splice(left + 1, 0 , marbleToAdd);
		curMarble = left + 1;
	}
	else{
		let r = (curMarble - 7) % marbles.length;
		r = (r < 0 ? marbles.length + r : r);
		scores[curPlayer] += marbleToAdd + marbles.splice(r, 1)[0];
		curMarble = r;
	}
	curPlayer = (curPlayer + 1 >= numPlayers ? 0 : curPlayer + 1);
}


console.log('P1: The winning Elf\'s score is ' + Math.max(...scores));
