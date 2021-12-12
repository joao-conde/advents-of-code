import { readFileSync } from "fs";

const graph: Record<string, string[]> = {};
const input = readFileSync("input/day12").toString().split("\n");
input
    .map(l => l.split("-"))
    .forEach(([src, dst]) => {
        if (!(src in graph)) graph[src] = [];
        graph[src].push(dst);

        if (!(dst in graph)) graph[dst] = [];
        graph[dst].push(src);
    });

const dfs = (cur: string, curPath: string[] = [], paths: string[][] = []) => {
    const next = graph[cur];

    const isLowerCase = cur.toLowerCase() === cur;
    if (isLowerCase && curPath.filter(c => c === cur).length >= 1) {
        return;
    }

    if (cur === "end") {
        paths.push(curPath.concat("end"));
        return;
    }

    next.forEach(n => {
        dfs(n, curPath.concat(cur), paths);
    });
};

const dfs2 = (allow: boolean, cur: string, curPath: string[] = [], paths: string[][] = []) => {
    let allowthis = allow;
    const next = graph[cur];

    const isLowerCase = cur.toLowerCase() === cur;
    if (isLowerCase && curPath.filter(c => c === cur).length >= 1) {
        if (!allowthis) return;
        allowthis = false;
    }

    if (cur === "start" && curPath.includes("start")) {
        return;
    }

    if (cur === "end") {
        paths.push(curPath.concat("end"));
        return;
    }

    next.forEach(n => {
        dfs2(allowthis, n, curPath.concat(cur), paths);
    });
};

const paths1: string[][] = [];
dfs("start", [], paths1);
console.log(paths1.length);

const paths: string[][] = [];
dfs2(true, "start", [], paths);
console.log(paths.length);
