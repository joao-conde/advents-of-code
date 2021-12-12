import { readFileAsString } from "./utils";

const commands: [string, number][] = readFileAsString("input/day02")
    .split("\n")
    .map(c => c.split(" "))
    .map(([dir, val]) => [dir, parseInt(val)]);

const [position, depth, aim] = commands.reduce(
    ([position, depth, aim], [dir, val]) => {
        if (dir === "forward") {
            position += val;
            depth += aim * val;
        }
        if (dir === "down") aim += val;
        if (dir === "up") aim -= val;
        return [position, depth, aim];
    },
    [0, 0, 0]
);
console.log("Part1:", position * aim);
console.log("Part2:", position * depth);
