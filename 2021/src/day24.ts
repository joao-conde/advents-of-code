import { range, readFileAsString } from "./utils";

class ALU {
    // eslint-disable-next-line
    [key: string]: number | any;

    constructor(w = 0, x = 0, y = 0, z = 0) {
        this.w = w;
        this.x = x;
        this.y = y;
        this.z = z;
    }

    copy = (): ALU =>
        new ALU(this.w as number, this.x as number, this.y as number, this.z as number);

    toString = (): string => `${this.w};${this.x};${this.y};${this.z}`;

    protected inp = (a: string, b: number) => (this[a] = b);

    protected add = (a: string, b: number | string) => (this[a] = this.val(a) + this.val(b));

    protected mul = (a: string, b: number | string) => (this[a] = this.val(a) * this.val(b));

    protected div = (a: string, b: number | string) =>
        (this[a] = Math.trunc(this.val(a) / this.val(b)));

    protected mod = (a: string, b: number | string) => (this[a] = this.val(a) % this.val(b));

    protected eql = (a: string, b: number | string) =>
        (this[a] = this.val(a) === this.val(b) ? 1 : 0);

    private val = (b: string | number): number => (typeof b === "number" ? b : this[b]);
}

const valid = (instructions: (string | number)[][], digits: number[]) => {
    const visited = new Set();

    let result: number | null = null;
    const best = (
        alu: ALU,
        instructions: (string | number)[][],
        step = 0,
        monad = ""
    ): number | null => {
        if (alu.z > Math.pow(10, 7)) return null;

        if (result) return result;

        if (step >= instructions.length) {
            result = alu.z === 0 ? parseInt(monad) : null;
            return result;
        }

        const [opcode, op1, op2] = instructions[step];
        if (opcode === "inp") {
            for (const digit of digits) {
                const nextalu = alu.copy();
                nextalu[opcode](op1 as string, digit);

                const k = `${nextalu.toString()};${monad.length}`;
                if (!visited.has(k)) {
                    best(nextalu, instructions, step + 1, monad + digit);
                    visited.add(k);
                }
            }
        } else {
            alu[opcode](op1, op2);
            best(alu, instructions, step + 1, monad);
        }

        return result;
    };

    best(new ALU(), instructions);

    return result;
};

const lines = readFileAsString("input/day24").split("\n");
const instructions = lines.map(line =>
    line.split(" ").map(x => {
        const num = parseInt(x);
        return isNaN(num) ? x : num;
    })
);

const p1 = valid(instructions, range(1, 10).reverse());
console.log("Part1:", p1);

const p2 = valid(instructions, range(1, 10));
console.log("Part2:", p2);
