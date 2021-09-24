const input = require('fs').readFileSync('../res/d07').toString().split('\n');

const WORKERS = 5, BASE_AMOUNT = 60;

const isSubset = (set1, set2) => {
    for (const el1 of set1) if (!set2.has(el1)) return false;
    return true;
};

const tasksRdyToExecute = (precedences, tasksDone) => {
    const tasks = [];
    Object.keys(precedences).forEach((k) => {
        if (isSubset(precedences[k], tasksDone)) tasks.push(k);
    });
    return tasks;
};

const taskTime = (task) => {
    return task.charCodeAt(0) - 'A'.charCodeAt(0) + 1 + BASE_AMOUNT;
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

const independentNodes = [...tasks].filter((node) => {
    return !Object.keys(precedences).includes(node);
});

const p1 = () => {
    const done = new Set();
    while (done.size < tasks.size) {
        const rdyToExecute = tasksRdyToExecute(precedences, done).concat(independentNodes);
        const notDone = rdyToExecute.filter((task) => {
            return !done.has(task);
        }).sort();
        done.add(notDone[0]);
    }
    console.log('P1 - Sequence of tasks: ' + [...done].join(''));
};

const p2 = () => {
    const done = new Set(), inProgress = {};
    let t = -1;
    while (done.size < tasks.size) {
        t++;
        Object.keys(inProgress).forEach((t) => {
            if (inProgress[t] == 1) {
                done.add(t);
                delete inProgress[t];
            } else inProgress[t]--;
        });

        const availableWorkers = WORKERS - Object.keys(inProgress).length;
        if (availableWorkers > 0) {
            const rdyToExecute = tasksRdyToExecute(precedences, done).concat(independentNodes);
            const notDone = rdyToExecute.filter((task) => {
                return !done.has(task) && !Object.keys(inProgress).includes(task);
            }).sort();

            const pick = (availableWorkers <= notDone.length ? availableWorkers : notDone.length);
            for (let i = 0; i < pick; i++) {
                inProgress[notDone[i]] = taskTime(notDone[i]);
            }
        }
    }
    console.log('P2 - Sequence of tasks: ' + [...done].join('') + ' done in ' + t + 's');
};

p1();
p2();
