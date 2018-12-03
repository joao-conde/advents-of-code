//Link to problem: https://adventofcode.com/2018/day/2
const input = require('fs').readFileSync('../res/d02').toString().split('\n');

buildOccurMap = (word) => {
	let map = {};
	word.split('').forEach(letter => letter in map ? map[letter]++ : map[letter] = 1);
	return map;
};

containsNOccur = (n, occurrencyMap) => {
	for (const key in occurrencyMap) {
		if (occurrencyMap[key] == n) return true;
	}
	return false;
}

getCommonSubstring = (str1, str2) => {
	const zip = str1.split('').map((e, i) => { return [e, str2.split('')[i]] });
	return zip.filter(e => { return e[0] == e[1] }).map(e => e[0]).join('');
};

let largestCommonStr = '';
input.forEach(str1 => input.forEach(str2 => {
	if (str1 != str2) {
		const common = getCommonSubstring(str1, str2);
		if (largestCommonStr.length < common.length) largestCommonStr = common;
	}
}));

console.log('Checksum is ' + input.filter(word => { return containsNOccur(2, buildOccurMap(word)) }).length
			* input.filter(word => { return containsNOccur(3, buildOccurMap(word)) }).length);

console.log('Common letters between correct box ID\'s are ' + largestCommonStr);