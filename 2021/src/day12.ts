import { readFileAsString } from "./utils";

type Graph = Record<string, string[]>;

const dfs = (
    graph: Graph,
    repeatSmall: boolean,
    cur = "start",
    curPath: string[] = []
): string[][] => {
    // successfully terminate this branch and add it to found paths
    if (cur === "end") return [curPath.concat(cur)];

    // prune branches going back to start
    if (cur === "start" && curPath.includes("start")) return [];

    // prune branches where a small node would be visited again
    const lowercase = cur === cur.toLowerCase();
    const frequency = curPath.filter(c => c === cur).length;
    if (lowercase && frequency >= 1) {
        if (repeatSmall) repeatSmall = false;
        else return [];
    }

    // recursive depth-first search call for each reachable node
    return graph[cur].reduce(
        (paths: string[][], next: string) =>
            paths.concat(dfs(graph, repeatSmall, next, curPath.concat(cur))),
        []
    );
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
console.log("Part1:", dfs(graph, false).length);
console.log("Part2:", dfs(graph, true).length);
