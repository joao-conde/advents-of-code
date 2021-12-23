import { readFileAsString, cartesian, range, JCSet as Set } from "./utils";

type Lights = Set<[number, number]>;

const LIGHT = "#";
const DARK = ".";

const countLit = (algorithm: string, image: string, steps: number, toggle: boolean): number => {
    let lights = image
        .split("\n")
        .map((row, x) =>
            row
                .split("")
                .map((c, y) => [x, y, c])
                .filter(([x, y, c]) => c === LIGHT)
        )
        .flat()
        .reduce((lights, [x, y]) => lights.add([x, y]), new Set()) as Lights;

    let spaceLit = false;
    range(steps).forEach(() => {
        lights = enhance(algorithm, lights, toggle && spaceLit);
        spaceLit = !spaceLit;
    });

    return lights.size;
};

const enhance = (algorithm: string, lights: Lights, spaceLit: boolean): Lights => {
    const { minx, miny, maxx, maxy } = getBounds(lights);
    const coords = cartesian(range(minx - 1, maxx + 2), range(miny - 1, maxy + 2));
    return coords.reduce((ls: Lights, [x, y]) => {
        const neighbors = [
            [x - 1, y - 1],
            [x - 1, y],
            [x - 1, y + 1],
            [x, y - 1],
            [x, y],
            [x, y + 1],
            [x + 1, y - 1],
            [x + 1, y],
            [x + 1, y + 1]
        ];

        const bin = neighbors.reduce((bin, [x, y]) => {
            const outofbounds = x < minx || x > maxx || y < miny || y > maxy;
            let pixel = "";
            if (outofbounds) {
                pixel = spaceLit ? "1" : "0";
            } else {
                pixel = lights.has([x, y]) ? "1" : "0";
            }
            return bin + pixel;
        }, "");

        const next = algorithm[parseInt(bin, 2)];
        if (next === LIGHT) ls.add([x, y]);
        return ls;
    }, new Set() as Lights);
};

const getBounds = (lights: Lights): { minx: number; miny: number; maxx: number; maxy: number } => {
    const values = lights.array();
    const minx = values.reduce((minx, [x, y]) => (x < minx ? x : minx), Infinity);
    const miny = values.reduce((miny, [x, y]) => (y < miny ? y : miny), Infinity);
    const maxx = values.reduce((maxx, [x, y]) => (x > maxx ? x : maxx), minx);
    const maxy = values.reduce((maxy, [x, y]) => (y > maxy ? y : maxy), miny);
    return { minx, miny, maxx, maxy };
};

const [algorithm, image] = readFileAsString("input/day20").split("\n\n");
const toggle = algorithm[0] === LIGHT && algorithm[algorithm.length - 1] === DARK;
console.log("Part1:", countLit(algorithm, image, 2, toggle));
console.log("Part2:", countLit(algorithm, image, 50, toggle));
