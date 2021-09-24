const input = require('fs').readFileSync('../res/d02').toString().split('\n');

const buildOccurMap = (word) => {
    const map = {};
    word.split('').forEach((letter) => letter in map ? map[letter]++ : map[letter] = 1);
    return map;
};

const containsNOccur = (n, occurrencyMap) => {
    for (const key in occurrencyMap) {
        if (occurrencyMap[key] == n) return true;
    }
    return false;
};

const getCommonSubstring = (str1, str2) => {
    const zip = str1.split('').map((e, i) => {
        return [e, str2.split('')[i]];
    });

    return zip.filter((e) => {
        return e[0] == e[1];
    }).map((e) => e[0]).join('');
};

const twos = input.filter((word) => {
    return containsNOccur(2, buildOccurMap(word));
}).length;

const threes = input.filter((word) => {
    return containsNOccur(3, buildOccurMap(word));
}).length;

console.log('P1 - Checksum is ' + twos * threes);

let largestCommonStr = '';
input.forEach((str1) => input.forEach((str2) => {
    if (str1 != str2) {
        const common = getCommonSubstring(str1, str2);
        if (largestCommonStr.length < common.length) largestCommonStr = common;
    }
}));

console.log('P2 - Common letters between correct box ID\'s are ' + largestCommonStr);
