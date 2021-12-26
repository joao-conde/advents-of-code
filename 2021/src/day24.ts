import { range, readFileAsString } from "./utils";

type ALURegister = number;

type ALUOperation = (a: string, b: number) => void;

class ALU {
    [key: string]: ALURegister | ALUOperation;

    constructor(w = 0, x = 0, y = 0, z = 0) {
        this.w = w;
        this.x = x;
        this.y = y;
        this.z = z;
    }

    toString = (): string => `${this.w};${this.x};${this.y};${this.z}`;

    copy = () => new ALU(this.w as number, this.x as number, this.y as number, this.z as number);

    inp = (a: string, b: number) => {
        this[a] = b;
    };

    add = (a: string, b: number | string) => {
        const value = typeof b === "number" ? b : this[b];
        this[a] = (this[a] as ALURegister) + (value as ALURegister);
    };

    mul = (a: string, b: number | string) => {
        const value = typeof b === "number" ? b : this[b];
        this[a] = (this[a] as ALURegister) * (value as ALURegister);
    };

    div = (a: string, b: number | string) => {
        const value = typeof b === "number" ? b : this[b];
        if (value === 0) console.log("dividing by zero wtf");
        this[a] = (this[a] as ALURegister) / (value as ALURegister);
    };

    mod = (a: string, b: number | string) => {
        const value = typeof b === "number" ? b : this[b];
        this[a] = (this[a] as ALURegister) % (value as ALURegister);
    };

    eql = (a: string, b: number | string) => {
        const value = typeof b === "number" ? b : this[b];
        this[a] = (this[a] as ALURegister) === (value as ALURegister) ? 1 : 0;
    };
}

const tryParseInt = (x: string): string | number => {
    const num = parseInt(x);
    return isNaN(num) ? x : num;
};

const digits = range(1, 10).reverse();
const DP: Record<string, number> = {};
const solve = (alu: ALU, instructions: (string | number)[][], step = 0, monad = "") => {
    if (step >= instructions.length) {
        if (alu.z === 0) console.log("valid MONAD:", monad);
        return alu.z === 0 ? parseInt(monad) : -1;
    }

    const [opcode, op1, op2] = instructions[step];
    if (opcode === "inp") {
        for (const digit of digits) {
            const nextalu = alu.copy();
            nextalu.inp(op1 as string, digit);

            const k = `${nextalu.toString()};${monad.length}`;
            if (!(k in DP)) {
                DP[k] = parseInt(monad);
                solve(nextalu, instructions, step + 1, monad + digit);
            }
        }
    } else {
        (alu[opcode] as ALUOperation)(op1 as string, op2 as number);
        solve(alu, instructions, step + 1, monad);
    }
};

const lines = readFileAsString("input/day24").split("\n");
const instructions = lines.map(line => line.split(" ").map(x => tryParseInt(x)));
const p1 = solve(new ALU(), instructions);
console.log(p1);
