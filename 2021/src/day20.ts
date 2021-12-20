import { readFileAsString, Set as Myset } from "./utils";
import { cartesian, range } from "./utils";

type Lights = Myset<[number, number]>;

const LIGHT = "#";

const getBounds = (lights: Lights): { minx: number; miny: number; maxx: number; maxy: number } => {
    const minx = lights.values().reduce((minx, [x, y]) => (x < minx ? x : minx), Infinity);
    const miny = lights.values().reduce((miny, [x, y]) => (y < miny ? y : miny), Infinity);
    const maxx = lights.values().reduce((maxx, [x, y]) => (x > maxx ? x : maxx), minx);
    const maxy = lights.values().reduce((maxy, [x, y]) => (y > maxy ? y : maxy), miny);
    return { minx, miny, maxx, maxy };
};

const enhance = (countDarks: boolean, algorithm: string, lights: Lights, darks: Lights): Lights => {
    const { minx, miny, maxx, maxy } = getBounds(lights);
    const coords = cartesian(range(minx - 1, maxx + 2), range(miny - 1, maxy + 2));

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
        ]
        .reduce((bin, [x, y]) => {
            const pixel = (lights.has([x, y]) || (countDarks && darks.has([x, y]))) ? "1" : "0";
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

const darks = image
.split("\n")
.map((row, x) =>
    row
        .split("")
        .map((c, y) => [x, y, c])
        .filter(([x, y, c]) => c === ".")
)
.flat()
.reduce((darks, [x, y]) => darks.add([x, y]), new Myset()) as Lights;

const lit = enhance(true, algorithm, enhance(false, algorithm, lights, darks), darks);
console.log("Part1:", lit.size());
