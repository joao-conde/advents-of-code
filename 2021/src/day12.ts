import { readFileAsString } from "./utils";

type Graph = Record<string, string[]>;

const dfs = (
    graph: Graph,
    smallVisited: boolean,
    cur = "start",
    curPath: string[] = [],
    paths: string[][] = []
): string[][] => {
    // successfully terminate this branch and add it to found paths
    if (cur === "end") {
        paths.push(curPath.concat(cur));
        return paths;
    }

    // prune branches going back to start
    if (cur === "start" && curPath.includes("start")) return paths;

    // prune branches where a small node would be visited again
    const lowercase = cur === cur.toLowerCase();
    const frequency = curPath.filter(c => c === cur).length;
    if (lowercase && frequency >= 1) {
        if (smallVisited) return paths;
        else smallVisited = true;
    }

    // recursive depth-first search call for each reachable node
    graph[cur].forEach(next => dfs(graph, smallVisited, next, curPath.concat(cur), paths));

    return paths;
};

const graph = readFileAsString("input/day12")
    .split("\n")
    .map(l => l.split("-"))
    .reduce((graph: Graph, [src, dst]) => {
        if (!(src in graph)) graph[src] = [];
        graph[src].push(dst);

        if (!(dst in graph)) graph[dst] = [];
        graph[dst].push(src);

        return graph;
    }, {});
console.log("Part1:", dfs(graph, true).length);
console.log("Part2:", dfs(graph, false).length);
