/* Link to problem: https://adventofcode.com/2018/day/7 */
const input = require('fs').readFileSync('../res/d07').toString().split('\n');

const isSubset = (set1, set2) => {
	let isSubset;
	set1.forEach((el1) => {
		isSubset = set2.has(el1);
	});
	return isSubset;
};

const tasksRdyToExecute = (precedences, tasksDone) => {
	const tasks = [];
	Object.keys(precedences).forEach((k) => {
		if (isSubset(precedences[k], tasksDone)) tasks.push(k);
	});
	return tasks;
};

const precedences = {}, tasks = new Set();
input.forEach((instr) => {
    const prec = instr.match(/^Step (\w)/)[1], task = instr.match(/step (\w)/)[1];
	tasks.add(prec).add(task);
	if (precedences[task]) {
		precedences[task].add(prec);
	} else {
		precedences[task] = new Set(prec);
	}
});

const possibleRootNodes = [...tasks].filter((node) => {
	return !Object.keys(precedences).includes(node);
});


const done = new Set();
while (done.size < tasks.size) {
	const rdyToExecute = tasksRdyToExecute(precedences, done).concat(possibleRootNodes);

	const notDone = rdyToExecute.filter((task) => {
		return !done.has(task);
	}).sort();

	done.add(notDone[0]);
}

console.log('P1 - Sequence of tasks: ' + [...done].join(''));