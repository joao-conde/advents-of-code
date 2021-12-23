import { readFileAsString } from "./utils";

const REGEX =
    /(?<action>on|off)? x=(?<x1>-?\d+)..(?<x2>-?\d+),y=(?<y1>-?\d+)..(?<y2>-?\d+),z=(?<z1>-?\d+)..(?<z2>-?\d+)/;

const input = readFileAsString("input/day22");
const steps = input
    .split("\n")
    .map(line => new RegExp(REGEX).exec(line)!.groups!)
    .map(group => ({
        action: group.action,
        x1: Number(group.x1),
        x2: Number(group.x2),
        y1: Number(group.y1),
        y2: Number(group.y2),
        z1: Number(group.z1),
        z2: Number(group.z2)
    }));

const cubes: Record<string, string> = {};
steps
    .filter(
        ({ action, x1, x2, y1, y2, z1, z2 }) =>
            x1 >= -50 && x2 <= 50 && y1 >= -50 && y2 <= 50 && z1 >= -50 && z2 <= 50
    )
    .forEach(({ action, x1, x2, y1, y2, z1, z2 }) => {
        for (let x = x1; x <= x2; x++) {
            for (let y = y1; y <= y2; y++) {
                for (let z = z1; z <= z2; z++) {
                    const k = JSON.stringify([x, y, z]);
                    if (!(k in cubes)) cubes[k] = "off";
                    cubes[k] = action;
                }
            }
        }
    });

console.log("Part1:", Object.values(cubes).filter(state => state === "on").length);
