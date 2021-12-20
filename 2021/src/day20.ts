import { cartesian, range, readFileAsString, Set as Myset } from "./utils";

type Lights = Myset<[number, number]>;

const LIGHT = "#";

const getPoints = (lights: Lights): [number, number][] => {
    const ps = lights
        .values()
        .map(([x, y]) => {
            const xs = range(x - 1, x + 2);
            const ys = range(y - 1, y + 2);
            return cartesian(xs, ys);
        })
        .flat() as [number, number][];
    return [...new Set(ps)];
};

const enhance = (algorithm: string, lights: Lights): Lights => {
    const coords = getPoints(lights);
    return coords.reduce((ls: Lights, [x, y]) => {
        const bin = [
            [x - 1, y - 1],
            [x - 1, y],
            [x - 1, y + 1],
            [x, y - 1],
            [x, y],
            [x, y + 1],
            [x + 1, y - 1],
            [x + 1, y],
            [x + 1, y + 1]
        ].reduce((bin, [x, y]) => {
            const pixel = lights.has([x, y]) ? "1" : "0";
            return bin + pixel;
        }, "");

        const i = parseInt(bin, 2);
        const next = algorithm[i];

        if (next === LIGHT) ls.add([x, y]);

        return ls;
    }, new Myset() as Lights);
};

const [algorithm, image] = readFileAsString("input/day20").split("\n\n");

const lights = image
    .split("\n")
    .map((row, x) =>
        row
            .split("")
            .map((c, y) => [x, y, c])
            .filter(([x, y, c]) => c === LIGHT)
    )
    .flat()
    .reduce((lights, [x, y]) => lights.add([x, y]), new Myset()) as Lights;

const lit = enhance(algorithm, enhance(algorithm, lights));
console.log("Part1:", lit.size());
