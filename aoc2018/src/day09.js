/* Link to problem: https://adventofcode.com/2018/day/9 */
const input = require('fs').readFileSync('../res/d09').toString();
const MULTIPLE = 23;
const numMarbles = parseInt(input.match('[0-9]*(?= points)')[0]); // not counting 0
const numPlayers = parseInt(input.match('[0-9]*(?= players)')[0]);

const insertBetween = (n1, n2, nn) => {
	n1.next = nn;
	nn.next = n2;
	n2.prev = nn;
	nn.prev = n1;
};

const remove = (n) => {
	n.prev.next = n.next;
	n.next.prev = n.prev;
};

const computeElfScore = (numPlayers, numMarbles) => {
	const scores = new Array(numPlayers).fill(0);
	let curPlayer = 0, curMarble = {val: 0, prev: null, next: null};
	curMarble.next = curMarble;
	curMarble.prev = curMarble;
	for (let marbleToAdd = 1; marbleToAdd <= numMarbles; marbleToAdd++) {
		if (marbleToAdd % MULTIPLE != 0) {
			const nextMarble = {val: marbleToAdd, prev: null, next: null};
			insertBetween(curMarble.next, curMarble.next.next, nextMarble);
			curMarble = nextMarble;
		} else {
			let toRemove = curMarble;
			for (let i = 0; i < 7; i++) toRemove = toRemove.prev;
			scores[curPlayer] += marbleToAdd + toRemove.val;
			curMarble = toRemove.next;
			remove(toRemove);
		}
		curPlayer = (curPlayer + 1 >= numPlayers ? 0 : curPlayer + 1);
	}
	return Math.max(...scores);
};

console.log('P1: Highest Elf\'s score: ' + computeElfScore(numPlayers, numMarbles));
console.log('P2: Highest Elf\'s score: ' + computeElfScore(numPlayers, numMarbles * 100));
