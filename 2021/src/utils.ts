import { readFileSync } from "fs";

export const cartesian = (...args: number[][]): number[][] =>
    args.reduce(
        (a: number[][], b) => a.map(x => b.map(y => x.concat(y))).reduce((a, b) => a.concat(b), []),
        [[]]
    );

export const copy = <T>(o: T): T => JSON.parse(JSON.stringify(o));

export const mean = (xs: number[]): number => sum(xs) / xs.length;

export const median = (xs: number[], sort = true): number => {
    const sorted = sort ? xs.sort((a, b) => a - b) : xs;
    const i = Math.floor(sorted.length / 2);
    return xs.length % 2 === 0 ? (sorted[i] + sorted[i - 1]) / 2 : sorted[i];
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

export const windows = <T>(xs: T[], size: number) =>
    xs.slice(0, -size + 1).map((_, i) => xs.slice(i, i + size));

export class JCSet<T> extends Set {
    private hash = (value: T): string => JSON.stringify(value);
    private dehash = (value: string): T => JSON.parse(value);

    override has = (value: T): boolean => super.has(this.hash(value));
    override add = (value: T): this => super.add(this.hash(value));
    override delete = (value: T): boolean => super.delete(this.hash(value));

    array = (): T[] => [...this].map(value => this.dehash(value));
}
