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

const metadataSum = (node) => {
    let sum = node.metadata.reduce((accumulator, currentValue) => accumulator + currentValue);
    node.children.forEach((element) => {
        sum += metadataSum(element);
    });
    return sum;
};

const findNodeValue = (node) => {
    if (node.children.length == 0) return metadataSum(node);

    let value = 0;
    node.metadata.forEach((element) => {
        if (element >= 1 && element <= node.children.length) {
            value += findNodeValue(node.children[element-1]);
        }
    });

    return value;
};

const tree = parseTree(1, input);
console.log('P1: The sum of all metadata entries is ' + metadataSum(tree));
console.log('P2: The value of the root node is ' + findNodeValue(tree));
