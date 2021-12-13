import { readFileSync } from "fs";

export const cartesian = (l1: number[], l2: number[]): number[][] =>
    l1.flatMap(x => l2.map(y => [x, y]));

export const copy = <T>(o: T): T => JSON.parse(JSON.stringify(o));

export const mean = (xs: number[]): number => sum(xs) / xs.length;

export const median = (xs: number[], sort = true): number => {
    const sorted = sort ? xs.sort((a, b) => a - b) : xs;
    const i1 = Math.floor(sorted.length / 2);
    const i2 = Math.ceil(sorted.length / 2);
    return (sorted[i1] + sorted[i2]) / 2;
};

export const mul = (xs: number[]): number => xs.reduce((sum, x) => sum * x, 1);

export const range = (a: number, b?: number, step = 1): number[] => {
    const size = Math.ceil(b !== undefined ? Math.abs(b - a) / Math.abs(step) : a / step);
    const offset = b !== undefined ? a : 0;
    return [...Array(size).keys()].map(i => offset + i * step);
};

export const readFileAsString = (path: string): string => readFileSync(path).toString();

export const scan = <T, X>(xs: Array<X>, fn: (state: T, next: X) => T, seed: T): T[] =>
    xs.reduce((states, x) => states.concat([fn(states[states.length - 1], x)]), [seed]);

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);

export class Set<T> {
    private storage: string[] = [];

    size = (): number => this.storage.length;

    hash = (value: T): string => JSON.stringify(value);

    dehash = (value: string): T => JSON.parse(value);

    has = (value: T): boolean => this.storage.includes(this.hash(value));

    add = (value: T): this => {
        if (!this.has(value)) this.storage.push(this.hash(value));
        return this;
    };

    delete = (value: T): this => {
        this.storage = this.storage.filter(t => t !== this.hash(value));
        return this;
    };

    values = (): T[] => this.storage.map(value => this.dehash(value));
}
