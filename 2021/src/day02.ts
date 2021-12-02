import { readFileSync } from "fs";

const directionTotal = (commands: [string, number][], command: string): number =>
    commands.filter(([dir, _]) => dir === command).reduce((sum, [_, val]) => sum + val, 0);

const input = readFileSync("input/day02").toString();
const commands: [string, number][] = input
    .split("\n")
    .map(c => c.split(" "))
    .map(([dir, val]) => [dir, parseInt(val)]);

const position = directionTotal(commands, "forward");
const downs = directionTotal(commands, "down");
const ups = directionTotal(commands, "up");
const depth1 = downs - ups;
console.log("Part1: " + depth1 * position);

const [depth2, _] = commands.reduce(
    ([depth, aim], [dir, val]) => {
        if (dir === "forward") depth += aim * val;
        if (dir === "down") aim += val;
        if (dir === "up") aim -= val;
        return [depth, aim];
    },
    [0, 0]
);
console.log("Part2: " + depth2 * position);
