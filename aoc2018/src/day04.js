/* Link to problem: https://adventofcode.com/2018/day/4 */
const input = require('fs').readFileSync('../res/d04').toString().split('\n').sort();

/*
 * Updates the current records for a guard that slept from [asleep, wakes[
 * 'records' is a map with guardID's to a 60 minute wide array with 1 point per day he slept on that minute
 */
const updateRecords = (records, guardID, asleep, wakes) => {
	if (!records[guardID]) records[guardID] = new Array(60).fill(0);
	for (let minute = asleep; minute < wakes; minute++) records[guardID][minute]++;
};

const records = {};
let currentGuardID = null, asleep = null;
input.forEach((record) => {
	switch (record.split(' ')[2]) {
	case 'Guard':
		currentGuardID = record.split(' ')[3];
		currentGuardID = currentGuardID.slice(1, currentGuardID.length);
		break;

	case 'falls':
		asleep = record.split(' ')[1].slice(0, -1).split(':')[1];
		break;

	case 'wakes':
		updateRecords(records, currentGuardID, parseInt(asleep), parseInt(record.split(' ')[1].slice(0, -1).split(':')[1]));
		break;
	}
});

/*
 * Identifies which guard splees more often and when (minute)
 * Identifies the guard that is most frequently asleep in a minute
 */
let sleepiestGuard = [undefined, 0, 0]; // guardID, totalSleep, minuteMaxSleep
let lazyHour = [undefined, 0, 0]; // guardID, freqAsleep, minute
for (const k in records) {
	const maxAsleep = Math.max(...records[k]);
	const minutesAsleep = records[k].reduce((acc, currVal) => acc + currVal);
	if (minutesAsleep > sleepiestGuard[1]) sleepiestGuard = [k, minutesAsleep, records[k].indexOf(maxAsleep)];
	if (maxAsleep > lazyHour[1]) lazyHour = [k, maxAsleep, records[k].indexOf(maxAsleep)];
}

console.log('P1: - ' + parseInt(sleepiestGuard[0]) * sleepiestGuard[2]);
console.log('P2: - ' + parseInt(lazyHour[0]) * lazyHour[2]);
