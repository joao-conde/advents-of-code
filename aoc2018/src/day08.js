/* Link to problem: https://adventofcode.com/2018/day/8 */
const input = require('fs').readFileSync('../res/d08').toString().split(' ').map((x) => parseInt(x));


const parseTree = (nodeID, info) => {
	const self = {
		id: nodeID,
		metadata: [],
		children: [],
	};

	const numChildren = info.splice(0, 1), numMetaInfo = info.splice(0, 1);

	for (let i = 0; i < numChildren; i++) {
		self.children.push(parseTree(nodeID + 1, info));
		nodeID += self.children.length;
	}

	for (let i = 0; i < numMetaInfo; i++) {
		self.metadata.push(info.splice(0, 1)[0]);
	}

	return self;
};

const metadataSum = (tree) => {
	let sum = tree.metadata.reduce((accumulator, currentValue) => accumulator + currentValue);
	tree.children.forEach((element) => {
		sum += metadataSum(element);
	});
	return sum;
};

const printTree = (tree) => {
	console.log('NODE ' + tree.id);
	if (tree.metadata.length != 0) {
		console.log('-Metadata-');
		tree.metadata.forEach((element) => {
			console.log(element);
		});
	}

	if (tree.children.length != 0) {
		console.log('-Children-');
		tree.children.forEach((element) => {
			printTree(element);
		});
	}
	console.log('END NODE ' + tree.id);
};


console.log('P1: The sum of all metadata entries is ' + metadataSum(parseTree(1, input)));
