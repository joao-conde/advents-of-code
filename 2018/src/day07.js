const input = require("fs").readFileSync("input/day07").toString().split("\n");

const WORKERS = 5;
const BASE_AMOUNT = 60;

const isSubset = (set1, set2) => {
    for (const el1 of set1) if (!set2.has(el1)) return false;
    return true;
};

const tasksRdyToExecute = (precedences, tasksDone) => {
    const tasks = [];
    Object.keys(precedences).forEach(k => {
        if (isSubset(precedences[k], tasksDone)) tasks.push(k);
    });
    return tasks;
};

const taskTime = task => task.charCodeAt(0) - "A".charCodeAt(0) + 1 + BASE_AMOUNT;

const precedences = {};
const tasks = new Set();
input.forEach(instr => {
    const prec = instr.match(/^Step (\w)/)[1];
    const task = instr.match(/step (\w)/)[1];
    tasks.add(prec).add(task);
    if (precedences[task]) {
        precedences[task].add(prec);
    } else {
        precedences[task] = new Set(prec);
    }
});

const independentNodes = [...tasks].filter(node => !Object.keys(precedences).includes(node));

const p1 = () => {
    const done = new Set();
    while (done.size < tasks.size) {
        const rdyToExecute = tasksRdyToExecute(precedences, done).concat(independentNodes);
        const notDone = rdyToExecute.filter(task => !done.has(task)).sort();
        done.add(notDone[0]);
    }
    console.log(`P1 - Sequence of tasks: ${[...done].join("")}`);
};

const p2 = () => {
    const done = new Set();
    const inProgress = {};
    let t = -1;
    while (done.size < tasks.size) {
        t += 1;
        Object.keys(inProgress).forEach(t => {
            if (inProgress[t] === 1) {
                done.add(t);
                delete inProgress[t];
            } else inProgress[t] -= 1;
        });

        const availableWorkers = WORKERS - Object.keys(inProgress).length;
        if (availableWorkers > 0) {
            const rdyToExecute = tasksRdyToExecute(precedences, done).concat(independentNodes);
            const notDone = rdyToExecute
                .filter(task => !done.has(task) && !Object.keys(inProgress).includes(task))
                .sort();

            const pick = availableWorkers <= notDone.length ? availableWorkers : notDone.length;
            for (let i = 0; i < pick; i += 1) {
                inProgress[notDone[i]] = taskTime(notDone[i]);
            }
        }
    }
    console.log(`P2 - Sequence of tasks: ${[...done].join("")} done in ${t}s`);
};

p1();
p2();
